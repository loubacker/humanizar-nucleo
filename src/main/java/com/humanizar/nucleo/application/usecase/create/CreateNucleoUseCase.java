package com.humanizar.nucleo.application.usecase.create;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRegisterDTO;
import com.humanizar.nucleo.application.mapper.NucleoCreateMapper;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Nucleo;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.springframework.stereotype.Component;

@Component
public class CreateNucleoUseCase {

    private final NucleoPort nucleoPort;
    private final NucleoCreateMapper nucleoCreateMapper;

    public CreateNucleoUseCase(NucleoPort nucleoPort, NucleoCreateMapper nucleoCreateMapper) {
        this.nucleoPort = nucleoPort;
        this.nucleoCreateMapper = nucleoCreateMapper;
    }

    public Nucleo execute(NucleoRegisterDTO nucleoRegisterDTO) {
        validatePayload(nucleoRegisterDTO);

        try {
            Nucleo nucleo = nucleoCreateMapper.toDomain(nucleoRegisterDTO);
            return nucleoPort.save(nucleo);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (RuntimeException runtimeException) {
            throw new NucleoException(ReasonCode.NUCLEO_CREATE_FAILED, runtimeException);
        }
    }

    private void validatePayload(NucleoRegisterDTO nucleoRegisterDTO) {
        if (nucleoRegisterDTO == null) {
            throw new NucleoException(ReasonCode.PAYLOAD_REQUIRED);
        }
        if (nucleoRegisterDTO.nome() == null || nucleoRegisterDTO.nome().isBlank()) {
            throw new NucleoException(ReasonCode.NOME_REQUIRED);
        }
        if (nucleoRegisterDTO.descricao() == null || nucleoRegisterDTO.descricao().isBlank()) {
            throw new NucleoException(ReasonCode.DESCRICAO_REQUIRED);
        }
    }
}
