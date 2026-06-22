<div align="center">
  <h1>Humanizar - Núcleo (Microservice)</h1>
  <p>Gestão dos Núcleos e Abordagens da clínica Humanizar, com responder de retrieve via mensageria.</p>

  <img alt="Java" src="https://img.shields.io/badge/Java-25-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white" />
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring_Boot-4.1.0-6DB33F?style=for-the-badge&logo=spring-boot&logoColor=white" />
  <img alt="GraalVM" src="https://img.shields.io/badge/GraalVM_Native-25-E76F00?style=for-the-badge&logo=oracle&logoColor=white" />
  <img alt="RabbitMQ" src="https://img.shields.io/badge/RabbitMQ-%23FF6600.svg?style=for-the-badge&logo=rabbitmq&logoColor=white" />
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" />
</div>

<br/>

Serviço REST síncrono responsável pelo CRUD de Núcleos e Abordagens e, adicionalmente, responder de retrieve resiliente via mensageria: consome comandos de consulta pelo RabbitMQ, lê o núcleo e devolve o resultado por outbox transacional (callback roteado dinamicamente). Protegido por OAuth2 JWT com RBAC, arquitetura Hexagonal e opção de runtime em binário nativo (GraalVM Native Image).

## Arquitetura e Padrões

- Arquitetura Hexagonal (`application`, `domain`, `infrastructure`).
- DTOs imutáveis com Java Records; mappers e use cases dedicados por operação.
- Adapters de domínio isolando JPA via ports.
- Controle de acesso por role via `@PreAuthorize` (RBAC).
- Responder de retrieve resiliente via mensageria: Outbox transacional para publicação confiável + Inbox idempotente (`processed_event`), espelhando o padrão do `patient-service`.
- ACK/NACK manual do RabbitMQ com política explícita por tipo de erro.
- Retry automático em falhas transientes nos endpoints GET (`@Retry`).
- Exception handler global com respostas padronizadas (`ReasonCode` seccionado por status HTTP).
- Execução otimizada com Virtual Threads e opção de binário nativo (GraalVM Native Image).

## Interfaces internas protegidas (REST)

Base path: `/api/v1`

### Núcleo
- `POST /nucleo/register` — cria núcleo. `ROLE_ADMINISTRADOR`.
- `GET /nucleos` — lista núcleos sem abordagens.
- `GET /nucleos/abordagens` — lista núcleos com abordagens.
- `GET /nucleo/{id}` — retorna núcleo por id.
- `PUT /nucleo/update/{id}` — atualiza núcleo. `ROLE_ADMINISTRADOR`.
- `DELETE /nucleo/delete/{id}` — remove núcleo. `ROLE_ADMINISTRADOR`. Rejeita com `409` se houver abordagens vinculadas.

### Abordagem
- `POST /abordagem/register` — cria abordagem. `ROLE_ADMINISTRADOR`.
- `GET /abordagem/{id}` — retorna abordagem por id.
- `PUT /abordagem/update/{id}` — atualiza abordagem. `ROLE_ADMINISTRADOR`.
- `DELETE /abordagem/delete/{id}` — remove abordagem. `ROLE_ADMINISTRADOR`.

Leitura requer apenas autenticação; escrita exige `ROLE_ADMINISTRADOR`.

## 🔄 Comunicação Assíncrona (RabbitMQ)

### Inbound — Comando de Retrieve

**Exchange `humanizar.nucleo.command`**
- `cmd.nucleo.retrieve.v1`

Fila principal:
- `humanizar.nucleo.retrieve`

DLQ:
- `humanizar.nucleo.retrieve.dlq`

Contrato consumido: `InboundEnvelopeDTO<NucleoRetrieveCommandDTO>` (carrega `nucleoId` + `callbackExchange`/`callbackRoutingKey` de retorno).

### Outbound — Resultado do Retrieve

**Exchange `humanizar.nucleo.event`**
- `ev.nucleo.retrieved.v1`
- `ev.nucleo.rejected.v1`

Contrato publicado: `OutboundEnvelopeDTO<T>` — `NucleoRetrievedDTO` (com o nome do núcleo) no sucesso; `NucleoRejectedDTO` com `NUCLEO_NOT_FOUND` quando o núcleo não existe. Publicação confiável via outbox; o callback é roteado dinamicamente para a exchange/routing key informadas no comando.

## ⛓️‍💥 Resiliência e Tolerância a Falhas

### ACK/NACK manual

`rabbitListenerContainerFactory` roda com `AcknowledgeMode.MANUAL` (configuração Java).

Política no consumer:
- `ack`: sucesso e evento duplicado.
- `nackRetry` (`requeue=true`): erro retentável.
- `nackDeadLetter` (`requeue=false`): parse inválido e erro não retentável.

Implementação central: `RabbitAcknowledgementConfig`.

### Retry transiente (endpoints GET)

`@Retry` via `ResilientMethodsConfig`.

- Max retries: 2, timeout: 30s.
- Predicate: exceções transientes de banco (`TransientDataAccessException`, `RecoverableDataAccessException`, `CannotCreateTransactionException`, `QueryTimeoutException`).

### Outbox states

- `NEW`
- `LOCKED`
- `PUBLISHED`
- `FAILED`
- `DEAD`

Com retentativa por `OutboxRetryPolicy` e controle de ownership/fencing por `instanceId`.

### ReasonCode

28 códigos com status HTTP, mensagem e flag `isRetryable()` por código (accessors duplos `httpStatus()`/`getStatusCode()`), seccionados por status — diferenciam falhas retentáveis (5xx transientes) de terminais (4xx).

### Idempotência

Antes de processar a regra, o consumer valida duplicidade via `ProcessedEventGuard`, que verifica a unicidade de `(consumer_name, event_id)` na tabela `processed_event`.

## 🛠️ Workers em Background

### OutboxRelayWorker (5s)

- Faz claim de eventos em `NEW`, `FAILED` e `LOCKED`.
- Aplica lock ownership por `instanceId` (fencing).
- Publica de forma assíncrona com controle de paralelismo.

### OutboxEventProcessor

Transições de status:
- sucesso: `LOCKED -> PUBLISHED`
- falha com retry: `LOCKED -> FAILED` (com `nextRetryAt`)
- tentativas esgotadas: `LOCKED -> DEAD`

### RetentionWorker (1h)

- Remove outbox antigo (`PUBLISHED` e `DEAD`) com mais de 48h.
- Remove `processed_event` antigo com mais de 90 dias.

## 🔐 Segurança

- API interna protegida por OAuth2 Resource Server JWT.
- JWK configurado por `AUTH_SERVER_URL`.
- RBAC: escrita exige `ROLE_ADMINISTRADOR`; leitura requer apenas autenticação.
- Sessão stateless, sem exposição de endpoint público (exceto `/actuator/**`).

## Estrutura do projeto

```text
src/main/java/com/humanizar/nucleo/
|-- application/
|   |-- catalog/                    # Exchange/Queue/RoutingKey/ConsumerCatalog
|   |-- dto/                        # DTOs imutaveis (nucleo, abordagem, envelope)
|   |-- mapper/                     # mappers DTO <-> dominio
|   |-- messaging/                  # inbound (retrieve) e outbound (resultado via outbox)
|   `-- service/                    # use cases CRUD (nucleo/abordagem) + retrieve
|-- domain/                         # modelos, enums, ports, exceptions
`-- infrastructure/                 # controllers, adapters, rabbit, outbox, persistence
```

### Pré-requisitos
- JDK 25
- Maven 3.9+
- PostgreSQL
- RabbitMQ

### Variáveis de Ambiente (`.env`)

```env
DB_URL=jdbc:postgresql://localhost:5432/db
DB_USERNAME=postgres
DB_PASSWORD=secret
RABBITMQ_URL=amqp://admin:admin@localhost:5672
AUTH_SERVER_URL=http://localhost:8080
```

### Execução local (JVM)

```bash
./mvnw clean install -DskipTests
./mvnw spring-boot:run
```

Porta padrão: `9094`
Health check: `http://localhost:9094/actuator/health`

## 🐳 Docker Native (GraalVM)

O Dockerfile do módulo usa build multi-stage com GraalVM Native Image:

1. Build stage (`ghcr.io/graalvm/native-image-community:25`) compila com:
   - `./mvnw -Pnative -DskipTests native:compile`
2. Runtime stage (`debian:bookworm-slim`) executa binário nativo:
   - `/app/app-binario`

Exemplo:

```bash
docker build -t humanizar-nucleo:native .
docker run --rm -p 9094:9094 --env-file .env humanizar-nucleo:native
```
