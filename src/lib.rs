#![forbid(unsafe_code)]
pub mod domain {
    pub mod model {
        mod abordagem;
        mod nucleo;
        pub use abordagem::Abordagem;
        pub use nucleo::Nucleo;
        pub mod messaging {
            mod nucleo_retrieve_command;
            mod outbox_event;
            mod outbox_message;
            mod processed_event;
            pub use nucleo_retrieve_command::{NucleoRetrieveCommand, ProcessingOutcome};
            pub use outbox_event::OutboxEvent;
            pub use outbox_message::OutboxMessage;
            pub use processed_event::ProcessedEvent;
        }
        pub mod enums {
            pub mod messaging {
                mod outbox_status;
                mod processed_result;
                pub use outbox_status::OutboxStatus;
                pub use processed_result::ProcessedResult;
            }
            mod reason_code;
            pub use reason_code::ReasonCode;
        }
    }

    pub mod exception {
        mod nucleo_exception;
        mod persistence_exception;
        mod reason_code_exception;
        mod security_exception;
        mod technical_error;
        pub use nucleo_exception::NucleoException;
        pub use persistence_exception::PersistenceException;
        pub use reason_code_exception::ReasonCodeException;
        pub use security_exception::SecurityException;
        pub use technical_error::TechnicalError;
    }

    pub mod port {
        pub mod messaging {
            mod message_publisher_port;
            mod outbox_event_port;
            mod processed_event_port;
            pub use message_publisher_port::MessagePublisherPort;
            pub use outbox_event_port::OutboxEventPort;
            pub use processed_event_port::ProcessedEventPort;
        }
        pub mod nucleo {
            mod nucleo_port;
            pub use nucleo_port::NucleoPort;
        }
        pub mod abordagem {
            mod abordagem_port;
            pub use abordagem_port::AbordagemPort;
        }
        pub use abordagem::AbordagemPort;
        pub use nucleo::NucleoPort;
    }
}
pub mod application {
    pub mod catalog {
        mod aggregate_catalog;
        mod consumer_catalog;
        mod exchange_catalog;
        mod producer_catalog;
        mod queue_catalog;
        mod routing_key_catalog;
        pub use aggregate_catalog::AggregateCatalog;
        pub use consumer_catalog::ConsumerCatalog;
        pub use exchange_catalog::ExchangeCatalog;
        pub use producer_catalog::ProducerCatalog;
        pub use queue_catalog::QueueCatalog;
        pub use routing_key_catalog::RoutingKeyCatalog;
    }

    pub mod messaging {
        pub mod inbound {
            pub mod dto {
                mod inbound_envelope_dto;
                pub use inbound_envelope_dto::{InboundEnvelopeDto, NucleoRetrieveCommandDto};
            }
            pub mod mapper {
                mod nucleo_command_mapper;
                pub use nucleo_command_mapper::NucleoCommandMapper;
            }
            pub mod handler {
                mod nucleo_retrieve_handler;
                pub use nucleo_retrieve_handler::NucleoRetrieveHandler;
            }
        }
        pub mod outbound {
            pub mod dto {
                mod outbound_envelope_dto;
                pub use outbound_envelope_dto::{
                    NucleoRejectedDto, NucleoRetrievedDto, OutboundEnvelopeDto,
                };
            }
            pub mod mapper {
                mod nucleo_callback_mapper;
                pub use nucleo_callback_mapper::NucleoCallbackMapper;
            }
            pub mod factory {
                mod nucleo_callback_factory;
                pub use nucleo_callback_factory::NucleoCallbackFactory;
            }
        }
    }

    pub mod http {
        pub mod dto {
            pub mod nucleo {
                mod nucleo_dto;
                pub use nucleo_dto::{NucleoRegisterDto, NucleoRetrieveDto, NucleoUpdateDto};
            }
            pub mod abordagem {
                mod abordagem_dto;
                pub use abordagem_dto::{
                    AbordagemRegisterDto, AbordagemRetrieveDto, AbordagemUpdateDto,
                };
            }
            pub use abordagem::{AbordagemRegisterDto, AbordagemRetrieveDto, AbordagemUpdateDto};
            pub use nucleo::{NucleoRegisterDto, NucleoRetrieveDto, NucleoUpdateDto};
        }
        pub mod mapper {
            pub mod nucleo {
                mod nucleo_mapper;
                pub use nucleo_mapper::NucleoMapper;
            }
            pub mod abordagem {
                mod abordagem_mapper;
                pub use abordagem_mapper::AbordagemMapper;
            }
            pub use abordagem::AbordagemMapper;
            pub use nucleo::NucleoMapper;
        }
    }
    pub(crate) mod validator {
        mod field_validator;
        pub(crate) use field_validator::required;
    }

    pub mod service {
        mod abordagem_service;
        mod nucleo_service;
        pub use abordagem_service::AbordagemService;
        pub use nucleo_service::NucleoService;
    }
}
pub mod infrastructure {
    pub mod server;
    pub mod messaging {
        mod messaging_services;
        pub use messaging_services::MessagingServices;
        pub mod inbound {
            pub(crate) mod idempotency {
                mod processed_event_guard;
                pub(crate) use processed_event_guard::ProcessedEventGuard;
            }

            pub mod rabbit {
                mod dead_letter_consumer;
                mod nucleo_retrieve_consumer;
                mod rabbit_runtime;
                pub use rabbit_runtime::RabbitRuntime;
            }
        }
        pub mod outbound {
            pub mod rabbit {
                mod rabbit_outbox_publisher;
                pub use rabbit_outbox_publisher::RabbitOutboxPublisher;
            }
            pub mod outbox {
                mod outbox_event_processor;
                pub(crate) mod outbox_relay_worker;
                pub(crate) use outbox_event_processor::OutboxEventProcessor;
                mod outbox_retry_policy;
                pub(crate) mod retention_worker;
                pub use outbox_retry_policy::OutboxRetryPolicy;
            }
        }
    }

    pub mod config {
        pub mod rabbit {
            mod rabbit_acknowledgement_config;
            mod rabbit_connection_config;
            mod rabbit_exchange_config;
            mod rabbit_queue_config;
            mod rabbit_settings;
            pub(crate) mod binding {
                mod rabbit_nucleo_binding_config;
                pub(crate) use rabbit_nucleo_binding_config::RabbitNucleoBindingConfig;
            }
            pub(crate) use rabbit_acknowledgement_config::RabbitAcknowledgementConfig;
            pub(crate) use rabbit_connection_config::{RabbitConnectionConfig, rabbit_error};
            pub(crate) use rabbit_exchange_config::RabbitExchangeConfig;
            pub(crate) use rabbit_queue_config::RabbitQueueConfig;
            pub use rabbit_settings::RabbitSettings;
        }
        mod database_config;
        mod environment_config;
        mod security_config;
        mod server_config;
        pub use database_config::{DatabaseConfig, DatabaseConfigError, DatabaseSettings};
        pub(crate) use environment_config::EnvironmentConfig;
        pub use security_config::{SecurityConfig, SecurityConfigError, SecuritySettings};
        pub use server_config::{ServerConfig, ServerConfigError};
    }

    pub(crate) mod adapter {
        mod outbox_event_adapter;
        mod processed_event_adapter;
        pub(crate) use outbox_event_adapter::OutboxEventAdapter;
        pub(crate) use processed_event_adapter::ProcessedEventAdapter;
        mod abordagem_adapter;
        mod nucleo_adapter;
        pub(crate) use abordagem_adapter::AbordagemAdapter;
        pub(crate) use nucleo_adapter::NucleoAdapter;
    }

    pub(crate) mod persistence {
        pub(crate) mod mapper {
            mod messaging_entity_mapper;
            pub(crate) use messaging_entity_mapper::MessagingEntityMapper;
        }
        pub(crate) mod entity {
            pub(crate) mod messaging {
                mod outbox_event_entity;
                mod processed_event_entity;
                pub(crate) use outbox_event_entity::OutboxEventEntity;
                pub(crate) use processed_event_entity::ProcessedEventEntity;
            }
            mod nucleo {
                mod nucleo_entity;
                pub(crate) use nucleo_entity::NucleoEntity;
            }
            mod abordagem {
                mod abordagem_entity;
                pub(crate) use abordagem_entity::AbordagemEntity;
            }
            pub(crate) use abordagem::AbordagemEntity;
            pub(crate) use nucleo::NucleoEntity;
        }
        pub(crate) mod repository {
            mod nucleo {
                mod nucleo_repository;
                pub(crate) use nucleo_repository::NucleoRepository;
            }
            mod abordagem {
                mod abordagem_repository;
                pub(crate) use abordagem_repository::AbordagemRepository;
            }
            mod messaging {
                mod outbox_event_repository;
                mod processed_event_repository;
                pub(crate) use outbox_event_repository::OutboxEventRepository;
                pub(crate) use processed_event_repository::ProcessedEventRepository;
            }
            pub(crate) use abordagem::AbordagemRepository;
            pub(crate) use messaging::{OutboxEventRepository, ProcessedEventRepository};
            pub(crate) use nucleo::NucleoRepository;
        }
    }

    pub(crate) mod handler {
        mod postgres_error_handler;
        pub(crate) use postgres_error_handler::{abordagem_create_error, database_error};
    }

    pub mod diagnostics {
        mod panic_diagnostics;
        pub use panic_diagnostics::install_panic_hook;
        mod safe_url;
        pub use safe_url::SafeUrl;
    }

    pub mod security {
        mod authenticated_user;
        mod authorization;
        mod jwks_cache;
        mod jwt_claims;
        mod jwt_validator;
        pub use authenticated_user::AuthenticatedUser;
        pub(crate) use authorization::{authenticate, require_administrator};
        pub(crate) use jwks_cache::{JwksCache, JwksHttpTimeouts};
        pub(crate) use jwt_validator::JwtValidator;
    }

    pub mod controller {
        pub mod extractor;
        mod http_observability;
        mod request_context;
        mod router;
        mod state;
        pub use request_context::RequestContext;
        pub use router::{create_router, create_router_with_limits};
        pub use state::ApplicationState;
        pub mod dto {
            mod error_response_dto;
            mod success_response_dto;
            pub use error_response_dto::ErrorResponseDto;
            pub(crate) use success_response_dto::SuccessObservation;
            pub use success_response_dto::SuccessResponseDto;
        }
        pub mod handler {
            mod http_error;
            pub(crate) use http_error::ErrorObservation;
            pub use http_error::{HttpError, status_for};
        }
        pub(crate) mod nucleo {
            pub(crate) mod nucleo_controller;
        }
        pub(crate) mod abordagem {
            pub(crate) mod abordagem_controller;
        }
    }
}
