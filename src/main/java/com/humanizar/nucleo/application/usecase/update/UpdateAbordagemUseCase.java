package com.humanizar.nucleo.application.usecase.update;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemUpdateDTO;
import com.humanizar.nucleo.application.mapper.AbordagemCreateMapper;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Abordagem;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class UpdateAbordagemUseCase {

    private final AbordagemPort abordagemPort;
    private final AbordagemCreateMapper abordagemCreateMapper;

    public UpdateAbordagemUseCase(AbordagemPort abordagemPort, AbordagemCreateMapper abordagemCreateMapper) {
        this.abordagemPort = abordagemPort;
        this.abordagemCreateMapper = abordagemCreateMapper;
    }

    public Abordagem execute(UUID abordagemId, AbordagemUpdateDTO abordagemUpdateDTO) {
        validatePayload(abordagemUpdateDTO);

        Abordagem abordagemAtual = abordagemPort.findById(abordagemId)
                .orElseThrow(() -> new NucleoException(ReasonCode.ABORDAGEM_NOT_FOUND));

        try {
            Abordagem abordagemAtualizada =
                    abordagemCreateMapper.toUpdatedDomain(abordagemId, abordagemUpdateDTO, abordagemAtual);
            return abordagemPort.save(abordagemAtualizada);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (RuntimeException runtimeException) {
            throw new NucleoException(ReasonCode.ABORDAGEM_UPDATE_FAILED, runtimeException);
        }
    }

    private void validatePayload(AbordagemUpdateDTO abordagemUpdateDTO) {
        if (abordagemUpdateDTO == null) {
            throw new NucleoException(ReasonCode.PAYLOAD_REQUIRED);
        }
        if (abordagemUpdateDTO.nome() == null || abordagemUpdateDTO.nome().isBlank()) {
            throw new NucleoException(ReasonCode.NOME_REQUIRED);
        }
        if (abordagemUpdateDTO.descricao() == null || abordagemUpdateDTO.descricao().isBlank()) {
            throw new NucleoException(ReasonCode.DESCRICAO_REQUIRED);
        }
    }
}
