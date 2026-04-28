package com.humanizar.nucleo.infrastructure.config;

import java.util.Collection;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Locale;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.method.configuration.EnableMethodSecurity;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.oauth2.jwt.Jwt;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationConverter;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.util.StringUtils;

@Configuration
@EnableWebSecurity
@EnableMethodSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        return http
                .csrf(AbstractHttpConfigurer::disable)
                .cors(Customizer.withDefaults())
                .sessionManagement(session -> session.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
                .authorizeHttpRequests(authorize -> authorize
                        .requestMatchers("/actuator/**").permitAll()
                        .requestMatchers(HttpMethod.OPTIONS, "/**").permitAll()
                        .requestMatchers("/api/v1/**").authenticated()
                        .anyRequest().denyAll()
                )
                .oauth2ResourceServer(oauth2 -> oauth2.jwt(jwt ->
                        jwt.jwtAuthenticationConverter(jwtAuthenticationConverter())))
                .build();
    }

    @Bean
    public JwtAuthenticationConverter jwtAuthenticationConverter() {
        JwtAuthenticationConverter converter = new JwtAuthenticationConverter();
        converter.setJwtGrantedAuthoritiesConverter(this::extractGrantedAuthorities);
        return converter;
    }

    private Collection<GrantedAuthority> extractGrantedAuthorities(Jwt jwt) {
        Set<String> roles = new LinkedHashSet<>();
        roles.addAll(extractRoleValues(jwt.getClaims().get("role")));
        roles.addAll(extractRoleValues(jwt.getClaims().get("roles")));

        return roles.stream()
                .filter(StringUtils::hasText)
                .map(this::normalizeRoleAuthority)
                .map(SimpleGrantedAuthority::new)
                .collect(Collectors.toList());
    }

    private List<String> extractRoleValues(Object claimValue) {
        if (claimValue == null) {
            return List.of();
        }

        if (claimValue instanceof String claimAsString) {
            if (claimAsString.contains(",")) {
                return List.of(claimAsString.split(","));
            }
            return List.of(claimAsString);
        }

        if (claimValue instanceof Collection<?> claimAsCollection) {
            return claimAsCollection.stream()
                    .map(String::valueOf)
                    .toList();
        }

        if (claimValue instanceof Object[] claimAsArray) {
            return Stream.of(claimAsArray)
                    .map(String::valueOf)
                    .toList();
        }

        return List.of(String.valueOf(claimValue));
    }

    private String normalizeRoleAuthority(String rawRole) {
        String normalizedRole = rawRole.trim();
        if (normalizedRole.startsWith("ROLE_")) {
            normalizedRole = normalizedRole.substring("ROLE_".length());
        }
        return "ROLE_" + normalizedRole.toUpperCase(Locale.ROOT);
    }
}
