package com.humanizar.nucleo.application.usecase.update;

import com.humanizar.nucleo.application.dto.nucleo.NucleoUpdateDTO;
import com.humanizar.nucleo.application.mapper.NucleoCreateMapper;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Nucleo;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class UpdateNucleoUseCase {

    private final NucleoPort nucleoPort;
    private final NucleoCreateMapper nucleoCreateMapper;

    public UpdateNucleoUseCase(NucleoPort nucleoPort, NucleoCreateMapper nucleoCreateMapper) {
        this.nucleoPort = nucleoPort;
        this.nucleoCreateMapper = nucleoCreateMapper;
    }

    public Nucleo execute(UUID nucleoId, NucleoUpdateDTO nucleoUpdateDTO) {
        validatePayload(nucleoUpdateDTO);

        Nucleo nucleoAtual = nucleoPort.findById(nucleoId)
                .orElseThrow(() -> new NucleoException(ReasonCode.NUCLEO_NOT_FOUND));

        try {
            Nucleo nucleoAtualizado = nucleoCreateMapper.toUpdatedDomain(nucleoId, nucleoUpdateDTO, nucleoAtual);
            return nucleoPort.save(nucleoAtualizado);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (RuntimeException runtimeException) {
            throw new NucleoException(ReasonCode.NUCLEO_UPDATE_FAILED, runtimeException);
        }
    }

    private void validatePayload(NucleoUpdateDTO nucleoUpdateDTO) {
        if (nucleoUpdateDTO == null) {
            throw new NucleoException(ReasonCode.PAYLOAD_REQUIRED);
        }
        if (nucleoUpdateDTO.nome() == null || nucleoUpdateDTO.nome().isBlank()) {
            throw new NucleoException(ReasonCode.NOME_REQUIRED);
        }
        if (nucleoUpdateDTO.descricao() == null || nucleoUpdateDTO.descricao().isBlank()) {
            throw new NucleoException(ReasonCode.DESCRICAO_REQUIRED);
        }
    }
}
