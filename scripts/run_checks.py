import argparse
import json
import os
from pathlib import Path
import subprocess
import time
import uuid


ROOT = Path(__file__).resolve().parents[1]


def docker(*arguments):
    return subprocess.check_output(["docker", *arguments], text=True, stderr=subprocess.STDOUT).strip()


def wait_ready(name, command):
    deadline = time.monotonic() + 120
    result = None
    while time.monotonic() < deadline:
        user = ["--user", "rabbitmq"] if command[0].startswith("rabbitmq") else []
        result = subprocess.run(["docker", "exec", *user, name, *command], capture_output=True)
        if result.returncode == 0:
            return
        time.sleep(1)
    detail = result.stderr.decode(errors="replace")[-2000:] if result else ""
    raise RuntimeError("Recurso isolado nao ficou disponivel: " + detail)


def run(command, environment):
    print("GATE " + " ".join(command), flush=True)
    subprocess.run(command, cwd=ROOT, env=environment, check=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--tests-only", action="store_true")
    arguments = parser.parse_args()
    environment = os.environ.copy()
    environment["CARGO_BUILD_JOBS"] = "4"
    if not arguments.tests_only:
        run(["cargo", "fmt", "--check"], environment)
        run(["cargo", "check", "--all-targets", "--locked"], environment)
        run(["cargo", "clippy", "--all-targets", "--locked", "--", "-D", "warnings"], environment)
    suffix = uuid.uuid4().hex[:16]
    names = []
    try:
        postgres = "codex-nucleo-pg-" + suffix
        names.append(postgres)
        docker("run", "--detach", "--name", postgres, "--label", "codex.task=nucleo-migration-" + suffix,
               "--publish", "127.0.0.1::5432", "--env", "POSTGRES_USER=nucleo_test",
               "--env", "POSTGRES_PASSWORD=nucleo_fixture_password", "--env", "POSTGRES_DB=nucleo_test", "postgres:17-alpine")
        wait_ready(postgres, ["pg_isready", "-U", "nucleo_test", "-d", "nucleo_test"])
        print("PostgreSQL isolado pronto", flush=True)
        port = docker("port", postgres, "5432/tcp").rsplit(":", 1)[1]
        environment["NUCLEO_TEST_DB_URL"] = "postgresql://127.0.0.1:" + port + "/nucleo_test"
        environment["NUCLEO_TEST_DB_USERNAME"] = "nucleo_test"
        environment["NUCLEO_TEST_DB_PASSWORD"] = "nucleo_fixture_password"
        rabbit = "codex-nucleo-rabbit-" + suffix
        names.append(rabbit)
        docker("run", "--detach", "--name", rabbit, "--label", "codex.task=nucleo-migration-" + suffix,
               "--publish", "127.0.0.1::5672", "--env", "ERL_FLAGS=+S 2:2 +A 2",
               "--env", "RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS=+S 2:2 +A 2", "--env", "RABBITMQ_CTL_ERL_ARGS=+S 1:1 +A 1",
               "--env", "RABBITMQ_DEFAULT_USER=nucleo_test", "--env", "RABBITMQ_DEFAULT_PASS=nucleo_fixture_password",
               "rabbitmq:4.3.2-management")
        wait_ready(rabbit, ["rabbitmq-diagnostics", "-q", "check_port_connectivity"])
        print("RabbitMQ isolado pronto", flush=True)
        rabbit_port = docker("port", rabbit, "5672/tcp").rsplit(":", 1)[1]
        environment["NUCLEO_TEST_RABBIT_URL"] = "amqp://nucleo_test:nucleo_fixture_password@127.0.0.1:" + rabbit_port + "/%2f"
        environment["NUCLEO_TEST_RESOURCE_ID"] = suffix
        state_directory = ROOT / ".test-resources"
        state_directory.mkdir(exist_ok=True)
        (state_directory / "owned-resources.json").write_text(json.dumps({"id": suffix, "containers": names}), encoding="utf-8")
        run(["cargo", "test", "--all-targets", "--locked"], environment)
    finally:
        for name in reversed(names):
            diagnostic = subprocess.run(["docker", "logs", "--tail", "60", name], capture_output=True, text=True)
            state_directory = ROOT / ".test-resources"
            state_directory.mkdir(exist_ok=True)
            (state_directory / (name + ".log")).write_text(diagnostic.stdout + diagnostic.stderr, encoding="utf-8")
            result = subprocess.run(["docker", "inspect", "--format", '{{index .Config.Labels "codex.task"}}', name], capture_output=True, text=True)
            if result.returncode == 0 and result.stdout.strip() == "nucleo-migration-" + suffix:
                subprocess.run(["docker", "rm", "--force", "--volumes", name], check=True, capture_output=True)


if __name__ == "__main__":
    main()
