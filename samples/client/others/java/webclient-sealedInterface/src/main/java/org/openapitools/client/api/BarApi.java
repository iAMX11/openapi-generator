package org.openapitools.client.api;

import org.openapitools.client.ApiClient;

import org.openapitools.client.model.Bar;
import org.openapitools.client.model.BarCreate;

import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Objects;
import java.util.Arrays;
import java.util.stream.Collectors;

import org.springframework.core.io.FileSystemResource;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.util.LinkedMultiValueMap;
import org.springframework.util.MultiValueMap;
import org.springframework.web.reactive.function.client.WebClient.ResponseSpec;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import reactor.core.publisher.Mono;
import reactor.core.publisher.Flux;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public class BarApi {
    private ApiClient apiClient;

    public BarApi() {
        this(new ApiClient());
    }

    public BarApi(ApiClient apiClient) {
        this.apiClient = apiClient;
    }

    public ApiClient getApiClient() {
        return apiClient;
    }

    public void setApiClient(ApiClient apiClient) {
        this.apiClient = apiClient;
    }

    /**
     * Create a Bar
     * 
     * <p><b>200</b> - Bar created
     * @param barCreate The barCreate parameter
     * @return Bar
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec createBarRequestCreation(@javax.annotation.Nonnull BarCreate barCreate) throws WebClientResponseException {
        Object postBody = barCreate;
        // verify the required parameter 'barCreate' is set
        if (barCreate == null) {
            throw new WebClientResponseException("Missing the required parameter 'barCreate' when calling createBar", HttpStatus.BAD_REQUEST.value(), HttpStatus.BAD_REQUEST.getReasonPhrase(), null, null, null);
        }
        // create path and map variables
        final Map<String, Object> pathParams = new HashMap<String, Object>();

        final MultiValueMap<String, String> queryParams = new LinkedMultiValueMap<String, String>();
        final HttpHeaders headerParams = new HttpHeaders();
        final MultiValueMap<String, String> cookieParams = new LinkedMultiValueMap<String, String>();
        final MultiValueMap<String, Object> formParams = new LinkedMultiValueMap<String, Object>();

        final String[] localVarAccepts = { 
            "application/json"
        };
        final List<MediaType> localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        final String[] localVarContentTypes = { 
            "application/json"
        };
        final MediaType localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);

        String[] localVarAuthNames = new String[] {  };

        ParameterizedTypeReference<Bar> localVarReturnType = new ParameterizedTypeReference<Bar>() {};
        return apiClient.invokeAPI("/bar", HttpMethod.POST, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * Create a Bar
     * 
     * <p><b>200</b> - Bar created
     * @param barCreate The barCreate parameter
     * @return Bar
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<Bar> createBar(@javax.annotation.Nonnull BarCreate barCreate) throws WebClientResponseException {
        ParameterizedTypeReference<Bar> localVarReturnType = new ParameterizedTypeReference<Bar>() {};
        return createBarRequestCreation(barCreate).bodyToMono(localVarReturnType);
    }

    /**
     * Create a Bar
     * 
     * <p><b>200</b> - Bar created
     * @param barCreate The barCreate parameter
     * @return ResponseEntity&lt;Bar&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<Bar>> createBarWithHttpInfo(@javax.annotation.Nonnull BarCreate barCreate) throws WebClientResponseException {
        ParameterizedTypeReference<Bar> localVarReturnType = new ParameterizedTypeReference<Bar>() {};
        return createBarRequestCreation(barCreate).toEntity(localVarReturnType);
    }

    /**
     * Create a Bar
     * 
     * <p><b>200</b> - Bar created
     * @param barCreate The barCreate parameter
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec createBarWithResponseSpec(@javax.annotation.Nonnull BarCreate barCreate) throws WebClientResponseException {
        return createBarRequestCreation(barCreate);
    }
}
