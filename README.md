<div align="center">
  <h1>Humanizar - Núcleo (Microservice)</h1>
  <p>Gestão dos Núcleos e Abordagens da clínica Humanizar.</p>

  <img alt="Java" src="https://img.shields.io/badge/Java-25-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white" />
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring_Boot-4.0.5-6DB33F?style=for-the-badge&logo=spring-boot&logoColor=white" />
  <img alt="GraalVM" src="https://img.shields.io/badge/GraalVM_Native-25-E76F00?style=for-the-badge&logo=oracle&logoColor=white" />
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" />
</div>

<br/>

Serviço REST síncrono responsável pelo CRUD de Núcleos e Abordagens. Protegido por OAuth2 JWT com RBAC, arquitetura Hexagonal e opção de runtime em binário nativo (GraalVM Native Image).

Página de serviços onde os núcleos são listados: [https://humanizar.aju.br/servicos](https://humanizar.feira.br/servicos)

## Arquitetura e Padrões

- Arquitetura Hexagonal (`application`, `domain`, `infrastructure`).
- DTOs imutáveis com Java Records.
- Mappers e use cases dedicados por operação.
- Adapters de domínio isolando JPA via ports.
- Exception handler global com respostas padronizadas.
- Controle de acesso por role via `@PreAuthorize` (RBAC).
- Retry automático em falhas transientes nos endpoints GET (`@Retry`).
- Execução otimizada com Virtual Threads e opção de binário nativo (GraalVM Native Image).

## Interfaces Internas Protegidas (REST)

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

## ⛓️‍💥 Resiliência

- `@Retry` (`ResilientMethodsConfig`) nos endpoints GET: máximo 2 tentativas, timeout 30s, predicate transiente de banco.
- Exception handler global retorna `ErrorResponseDTO` com status, reason code, mensagem, path e timestamp.

## 🔐 Segurança

- OAuth2 Resource Server JWT, JWK configurado por `AUTH_SERVER_URL`.
- RBAC: escrita exige `ROLE_ADMINISTRADOR`; leitura requer apenas autenticação.
- Sessão stateless, sem exposição de endpoint público.

## Estrutura do Projeto

```text
src/main/java/com/humanizar/nucleo/
|-- application/                   # dto, mapper, service, usecase
|-- domain/                        # model, enums, exception, port
`-- infrastructure/                # adapter, config, controller, persistence
```

## Como executar localmente

### Pré-requisitos
- JDK 25
- Maven 3.9+
- PostgreSQL

### Variáveis de Ambiente (`.env`)

```env
DB_URL=jdbc:postgresql://localhost:5432/db
DB_USERNAME=postgres
DB_PASSWORD=secret
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
