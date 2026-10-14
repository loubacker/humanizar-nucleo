<div align="center">
  <h1>Humanizar - Núcleo (Microservice)</h1>
  <p>Gestão de núcleos e abordagens da Humanizar em Rust, com API interna protegida, PostgreSQL assíncrono e mensageria RabbitMQ.</p>

  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.99-000000?style=for-the-badge&logo=rust&logoColor=white" />
  <img alt="Axum" src="https://img.shields.io/badge/Axum-0.8-7B1FA2?style=for-the-badge&logo=rust&logoColor=white" />
  <img alt="Tokio" src="https://img.shields.io/badge/Tokio-1.53-2C5BB4?style=for-the-badge&logo=rust&logoColor=white" />
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" />
  <img alt="RabbitMQ" src="https://img.shields.io/badge/RabbitMQ-FF6600?style=for-the-badge&logo=rabbitmq&logoColor=white" />
  <img alt="Docker" src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" />
  <img alt="Keycloak" src="https://img.shields.io/badge/Keycloak-4D4D4D?style=for-the-badge&logo=keycloak&logoColor=white" />
</div>

<br/>

O `humanizar-nucleo` é um serviço interno do ecossistema Humanizar. Ele mantém
o catálogo de núcleos e suas abordagens e responde a comandos de consulta pelo
RabbitMQ. A implementação Rust preserva identidades, vínculos e contratos de
mensageria do serviço Java.

O runtime usa Axum e Tokio, persistência PostgreSQL assíncrona e composição
explícita de dependências segundo Arquitetura Hexagonal. As respostas HTTP
usam envelopes de sucesso e erro ligados à correlação e à observabilidade.

## Responsabilidades

- cadastro, consulta, atualização e exclusão de núcleos;
- cadastro e manutenção das abordagens vinculadas a cada núcleo;
- proteção da integridade dos vínculos entre núcleo e abordagem;
- consulta do nome de núcleo por comando RabbitMQ;
- persistência atômica dos recibos de processamento e callbacks na outbox;
- rastreabilidade das operações por código, correlação e resultado.

## Arquitetura e Padrões

- camadas `application`, `domain` e `infrastructure` com dependências orientadas
  para o domínio;
- ports de I/O assíncronos `Send + Sync`, injetados como `Arc<dyn Port>`;
- DTOs separados por boundary HTTP e mensageria;
- mappers manuais, campo a campo;
- entities escalares e repositories responsáveis pelo SQL parametrizado;
- pool PostgreSQL assíncrono com BB8 e validação da conexão no checkout;
- erros tipados, `ReasonCode` estável e tradução HTTP na infraestrutura;
- transactional outbox, idempotência e workers supervisionados;
- módulos registrados no `lib.rs`, sem `mod.rs`;
- testes externos em `tests/`;
- código próprio protegido por `#![forbid(unsafe_code)]`, sem FFI.

## Regras de Domínio

### Núcleo

- Nome e descrição são obrigatórios.
- O servidor define o identificador durante a criação.
- Atualizações preservam o identificador e a data de criação.
- Núcleos com abordagens vinculadas não podem ser excluídos.

### Abordagem

- Nome, descrição e `nucleoId` são obrigatórios no cadastro.
- O núcleo informado precisa existir.
- Atualizações preservam o núcleo de origem, o identificador e a data de criação.
- A FK protege o vínculo também sob concorrência entre cadastro e exclusão.

## API HTTP

Base path: `/api/v1`.

### Saúde

| Método | Rota | Autenticação |
| --- | --- | --- |
| `GET` | `/health` | Não |

O healthcheck retorna `200` com `{"status":"UP"}` quando a mensageria está
pronta e a consulta PostgreSQL conclui dentro do orçamento de 2 segundos.
Indisponibilidade retorna `503` com erro padronizado e correlação.

### Consultas

| Método | Rota | Descrição |
| --- | --- | --- |
| `GET` | `/api/v1/nucleos` | Lista núcleos com a coleção de abordagens vazia |
| `GET` | `/api/v1/nucleos/abordagens` | Lista núcleos com suas abordagens, carregadas em lote |
| `GET` | `/api/v1/nucleo/{id}` | Consulta um núcleo com suas abordagens |
| `GET` | `/api/v1/nucleos/{id}` | Alias da consulta individual de núcleo |
| `GET` | `/api/v1/abordagem/{id}` | Consulta uma abordagem |

### Comandos

| Método | Rota | Resultado |
| --- | --- | --- |
| `POST` | `/api/v1/nucleo/register` | Cria núcleo, HTTP `201` |
| `PUT` | `/api/v1/nucleo/update/{id}` | Atualiza núcleo, HTTP `200` |
| `DELETE` | `/api/v1/nucleo/delete/{id}` | Exclui núcleo, HTTP `200` |
| `POST` | `/api/v1/abordagem/register` | Cria abordagem, HTTP `201` |
| `PUT` | `/api/v1/abordagem/update/{id}` | Atualiza abordagem, HTTP `200` |
| `DELETE` | `/api/v1/abordagem/delete/{id}` | Exclui abordagem, HTTP `200` |

As consultas exigem `Authorization: Bearer <token>`. As mutações exigem
também a role `ADMINISTRADOR`.

### Respostas e correlação

Respostas de negócio bem-sucedidas usam `SuccessResponseDto`:

```json
{
  "code": "NUCLEO_DELETED",
  "success": true,
  "correlationId": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Registro excluido com sucesso.",
  "data": null,
  "timestamp": "2026-10-11T14:30:00Z"
}
```

Consultas, criações e atualizações usam `data` para transportar o DTO ou a
lista. Exclusões retornam `data: null`.

Falhas usam `ErrorResponseDto` com `timestamp`, `status`, `error`, `reasonCode`,
`message`, `path` e `correlationId`. Erros de JSON, UUID, limites, rota e método
também passam pelo handler.

O header `X-Correlation-ID`, o corpo e o log usam o mesmo UUID. Quando o
header está ausente ou inválido, o serviço gera uma correlação.

## 🔐 Segurança

- autenticação stateless por Bearer JWT;
- algoritmo permitido `RS256` e validação de `kid`, assinatura, `exp`, `nbf`,
  `iss`, `aud` e `sub`;
- administração concedida exclusivamente por `realm_access.roles`, preservando
  a semântica de case e prefixo do Java;
- claims `role` e `roles` isolados não concedem privilégios;
- JWKS carregado da URL completa de `KEYCLOAK_ISSUER`, sem acrescentar sufixos;
- URL JWKS sem credenciais, query ou fragmento;
- redirects recusados e resposta JWKS limitada a 1 MiB;
- chaves expiradas não são reutilizadas quando o refresh falha;
- CORS para os origins HTTP locais e da rede privada configurados no router;
- credenciais, Bearer, conteúdo do JWT, payloads e causas técnicas brutas fora
  dos logs da aplicação.

## ⛓️‍💥 Resiliência e Observabilidade

### HTTP e PostgreSQL

Mutações HTTP não têm retry automático. Um timeout não comprova ausência de
efeito nem rollback da intenção. O prazo HTTP padrão é 30 segundos, e o corpo
JSON é limitado a 2 MiB.

O pool mantém os padrões do Núcleo: mínimo de 3 conexões, máximo de 10,
aquisição em até 30 segundos, idle de 300 segundos e lifetime de 600 segundos.
As sessões usam `statement_timeout=25000`, `lock_timeout=10000` e
`idle_in_transaction_session_timeout=25000`, em milissegundos.

### ReasonCode

| Status | Exemplos de ReasonCode |
| --- | --- |
| `400` | `PAYLOAD_REQUIRED`, `REQUEST_BODY_INVALID`, `REQUEST_PARAMETER_INVALID`, `NOME_REQUIRED`, `DESCRICAO_REQUIRED`, `NUCLEO_ID_REQUIRED` |
| `401` | `AUTHENTICATION_REQUIRED` |
| `403` | `AUTHORIZATION_DENIED` |
| `404` | `NUCLEO_NOT_FOUND`, `ABORDAGEM_NOT_FOUND`, `ROUTE_NOT_FOUND` |
| `405` | `METHOD_NOT_ALLOWED` |
| `409` | `NUCLEO_HAS_ABORDAGENS` e falhas contextuais de criação, atualização ou exclusão |
| `413` | `PAYLOAD_TOO_LARGE` |
| `500` | `DATABASE_ACCESS_FAILED`, `UNEXPECTED_ERROR` |
| `502` | `INTEGRATION_FAILURE` |
| `503` | `PERSISTENCE_FAILURE`, `MESSAGING_UNAVAILABLE` |
| `504` | `REQUEST_TIMEOUT` |

Logs HTTP relacionam código, operação, fase, resultado, correlação, status e
duração. Rejeições `4xx` usam `warn`; falhas `5xx` usam `error`. As causas são
preservadas internamente, sem expor seu texto no JSON público. Sucessos de
`/health` permanecem silenciosos; falhas continuam observáveis.

## Mensageria

### Exchanges

```text
humanizar.nucleo.command
humanizar.nucleo.event
```

### Comando consumido

```text
cmd.nucleo.retrieve.v1
```

O envelope v1 usa camelCase. O payload exige `sourceEventId`, `nucleoId`,
`callbackExchange` e `callbackRoutingKey`. O último campo permanece obrigatório
por compatibilidade; os callbacks usam as routing keys fixas abaixo.

### Callbacks publicados

```text
ev.nucleo.retrieved.v1
ev.nucleo.rejected.v1
```

A ausência de núcleo produz um callback rejected com `NUCLEO_NOT_FOUND`.
A consulta foi processada, portanto seu recibo permanece `SUCCESS`.

### Filas e processamento

```text
humanizar.nucleo.retrieve
humanizar.nucleo.retrieve.dlq
```

As filas são quorum queues, com ACK manual, delivery limit padrão de 3 e DLQ
dedicada. O consumer lógico é `nucleo.retrieve.consumer`. A DLQ mantém a
política legada de diagnóstico e descarte por ACK.

Consulta, recibo e callback-outbox usam a mesma transação. A idempotência usa
`(consumer_name, event_id)`, protegida por advisory lock e unique. ACK ocorre
após commit. Rejeições terminais identificáveis geram recibo `FAILED` antes de
NACK; replay recebe ACK. Falhas transitórias permanecem elegíveis a retry.

### Outbox e retenção

O relay da outbox:

1. reivindica lotes com `FOR UPDATE SKIP LOCKED`;
2. usa `next_retry_at` como lease e `locked_by` como proprietário;
3. publica fora da transação, com mandatory, persistência e publisher confirm;
4. mantém event ID e bytes persistidos nas repetições;
5. condiciona a atualização final ao proprietário e ao estado `LOCKED`;
6. aplica backoff exponencial e move tentativas esgotadas para `DEAD`.

Padrões: prefetch 20, dois consumers, batch 20, paralelismo 8, poll de 5000 ms,
lease de 300 s, confirm de 5 s e cinco tentativas. A base de retry é 2000 ms,
limitada a 300000 ms; a primeira falha aguarda 4000 ms.

A retenção roda a cada hora, em lotes de até 1000 registros por tabela.
Eventos `PUBLISHED` e `DEAD` usam `created_at` anterior a 48 horas; recibos usam
`processed_at` anterior a 90 dias. Recibos ligados à outbox ativa são protegidos.
O campo opcional `event_hash` não altera a identidade de deduplicação.

## Persistência

| Arquivo | Finalidade |
| --- | --- |
| [0001_initial_schema.sql](migrations/0001_initial_schema.sql) | Criação completa das tabelas, PKs, FK, UKs, CHECKs e índices em banco vazio |

As tabelas permanecem `nucleo`, `abordagem`, `outbox_event` e `processed_event`.
O script único é transacional e deve ser aplicado em banco vazio. A FK
`fk_abordagem_nucleo` garante o vínculo 1:N e restringe a exclusão de núcleos
com abordagens. As duas UKs protegem a identidade dos eventos e dos recibos;
cinco CHECKs validam versão, estados e contadores de mensageria.

Os oito índices de consulta incluem vínculos, despacho, correlação, agregado,
evento, processamento e reason code. O despacho e a retenção usam `id` como
desempate nos respectivos índices, sem índices equivalentes duplicados.

O processo não executa DDL automaticamente. No startup, verifica as colunas,
a FK e as duas proteções únicas por consultas incorporadas ao código Rust.

Timestamps de negócio e mensageria mantêm `TIMESTAMP` sem fuso e
`NaiveDateTime`, compatíveis com `LocalDateTime`. Novos valores seguem a hora
local do processo; a implantação deve usar o mesmo fuso do serviço legado.
O timestamp diagnóstico dos envelopes HTTP é um instante UTC.

## Diagnóstico de Startup

O bootstrap segue esta ordem:

1. carregamento e validação do ambiente e dos parâmetros;
2. inicialização do pool PostgreSQL;
3. verificação das colunas e constraints do schema;
4. carregamento do JWKS e composição da segurança;
5. criação do listener e inicialização da mensageria;
6. composição das rotas e liberação do atendimento HTTP.

Falhas de configuração encerram o processo com código `1`. Diagnósticos usam
mensagens controladas, sem imprimir credenciais, linhas do `.env` ou payloads
de panic. Workers têm supervisão e encerramento com orçamento explícito.

## Como Executar Localmente

### Pré-requisitos

- Rust `1.99.0`, edition 2024, com Cargo, Clippy e rustfmt;
- PostgreSQL com o schema compatível preparado;
- Keycloak com JWKS disponível;
- RabbitMQ com acesso à topologia do serviço;
- `.env` preenchido para o ambiente de execução.

### Execução local

```bash
cargo run --locked
```

Porta padrão: `9094`.

Healthcheck: `http://localhost:9094/health`.

### Quality gates

```bash
cargo fmt --check
cargo check --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
```

Para a suíte completa, incluindo recursos isolados de PostgreSQL e RabbitMQ:

```bash
python -B scripts/run_checks.py
```

O runner também executa `cargo test --all-targets --locked`. Ele cria
containers exclusivos em loopback, usa credenciais sintéticas e remove apenas
os recursos daquela execução. Testes de integração sem as variáveis do runner
falham explicitamente.

Os scanners arquiteturais verificam a estrutura separadamente dos testes
executáveis. Verificações locais não comprovam CI, Native Image, build Linux
ou comportamento de produção.

## 🐳 Docker

O [Dockerfile](Dockerfile) usa build multi-stage:

1. `rust:1.99.0-bookworm` compila o binário release com `--locked`;
2. `debian:bookworm-slim` executa `/app/humanizar-nucleo` com usuário não root;
3. o healthcheck consulta `/health` a cada 30 segundos.

```bash
docker build -t humanizar-nucleo .
docker run --rm -p 9094:9094 --env-file .env humanizar-nucleo:latest
```

O `.env` não é copiado para a imagem. As variáveis chegam por `--env-file` ou
pelo orquestrador; os destinos precisam ser acessíveis a partir do container.

## Integração com o Admin Gateway

O Gateway adaptado exige os envelopes Rust, valida status e código esperados,
extrai `data` e encaminha a correlação explicitamente. As respostas públicas
para o dashboard mantêm dados diretos; DELETE retorna JSON com `message` para
o consumidor que executa `JSON.parse`.

A transição do serviço e do Gateway precisa ser coordenada. A aplicação do
script inicial em banco vazio e a troca de tráfego são etapas operacionais
separadas da implementação local.
