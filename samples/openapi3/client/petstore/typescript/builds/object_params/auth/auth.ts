import { RequestContext } from "../http/http";

/**
 * Interface authentication schemes.
 */
export interface SecurityAuthentication {
    /*
     * @return returns the name of the security authentication as specified in OAI
     */
    getName(): string;

    /**
     * Applies the authentication scheme to the request context
     *
     * @params context the request context which should use this authentication scheme
     */
    applySecurityAuthentication(context: RequestContext): void | Promise<void>;
}

export interface TokenProvider {
  getToken(): Promise<string> | string;
}

/**
 * Applies oauth2 authentication to the request context.
 */
export class PetstoreAuthAuthentication implements SecurityAuthentication {
    /**
     * Configures OAuth2 with the necessary properties
     *
     * @param accessToken: The access token to be used for every request
     */
    public constructor(private accessToken: string) {}

    public getName(): string {
        return "petstore_auth";
    }

    public applySecurityAuthentication(context: RequestContext) {
        context.setHeaderParam("Authorization", "Bearer " + this.accessToken);
    }
}

/**
 * Applies apiKey authentication to the request context.
 */
export class ApiKeyAuthentication implements SecurityAuthentication {
    /**
     * Configures this api key authentication with the necessary properties
     *
     * @param apiKey: The api key to be used for every request
     */
    public constructor(private apiKey: string) {}

    public getName(): string {
        return "api_key";
    }

    public applySecurityAuthentication(context: RequestContext) {
        context.setHeaderParam("api_key", this.apiKey);
    }
}

export type AuthMethods = {
    "default"?: SecurityAuthentication,
    "petstore_auth"?: SecurityAuthentication,
    "api_key"?: SecurityAuthentication
}

export type ApiKeyConfiguration = string;
export type HttpBasicConfiguration = { "username": string, "password": string };
export type HttpBearerConfiguration = { tokenProvider: TokenProvider };
export type OAuth2Configuration = { accessToken: string };
export type HttpSignatureConfiguration = unknown; // TODO: Implement

export type AuthMethodsConfiguration = {
    "default"?: SecurityAuthentication,
    "petstore_auth"?: OAuth2Configuration,
    "api_key"?: ApiKeyConfiguration
}

/**
 * Creates the authentication methods from a swagger description.
 *
 */
export function configureAuthMethods(config: AuthMethodsConfiguration | undefined): AuthMethods {
    let authMethods: AuthMethods = {}

    if (!config) {
        return authMethods;
    }
    authMethods["default"] = config["default"]

    if (config["petstore_auth"]) {
        authMethods["petstore_auth"] = new PetstoreAuthAuthentication(
            config["petstore_auth"]["accessToken"]
        );
    }

    if (config["api_key"]) {
        authMethods["api_key"] = new ApiKeyAuthentication(
            config["api_key"]
        );
    }

    return authMethods;
}