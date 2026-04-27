package com.humanizar.nucleo.application.usecase.create;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.mapper.AbordagemCreateMapper;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Abordagem;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.springframework.stereotype.Component;

@Component
public class CreateAbordagemUseCase {

    private final AbordagemPort abordagemPort;
    private final NucleoPort nucleoPort;
    private final AbordagemCreateMapper abordagemCreateMapper;

    public CreateAbordagemUseCase(AbordagemPort abordagemPort,
                                  NucleoPort nucleoPort,
                                  AbordagemCreateMapper abordagemCreateMapper) {
        this.abordagemPort = abordagemPort;
        this.nucleoPort = nucleoPort;
        this.abordagemCreateMapper = abordagemCreateMapper;
    }

    public Abordagem execute(AbordagemRegisterDTO abordagemRegisterDTO) {
        validatePayload(abordagemRegisterDTO);
        validateNucleo(abordagemRegisterDTO);

        try {
            Abordagem abordagem = abordagemCreateMapper.toDomain(abordagemRegisterDTO);
            return abordagemPort.save(abordagem);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (RuntimeException runtimeException) {
            throw new NucleoException(ReasonCode.ABORDAGEM_CREATE_FAILED, runtimeException);
        }
    }

    private void validatePayload(AbordagemRegisterDTO abordagemRegisterDTO) {
        if (abordagemRegisterDTO == null) {
            throw new NucleoException(ReasonCode.PAYLOAD_REQUIRED);
        }
        if (abordagemRegisterDTO.nome() == null || abordagemRegisterDTO.nome().isBlank()) {
            throw new NucleoException(ReasonCode.NOME_REQUIRED);
        }
        if (abordagemRegisterDTO.descricao() == null || abordagemRegisterDTO.descricao().isBlank()) {
            throw new NucleoException(ReasonCode.DESCRICAO_REQUIRED);
        }
        if (abordagemRegisterDTO.nucleoId() == null) {
            throw new NucleoException(ReasonCode.NUCLEO_ID_REQUIRED);
        }
    }

    private void validateNucleo(AbordagemRegisterDTO abordagemRegisterDTO) {
        if (nucleoPort.findById(abordagemRegisterDTO.nucleoId()).isEmpty()) {
            throw new NucleoException(ReasonCode.NUCLEO_NOT_FOUND);
        }
    }
}
