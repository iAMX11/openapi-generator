use bytes::Bytes;
use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use http_body_util::{combinators::BoxBody, Full};
use hyper::{body::{Body, Incoming}, HeaderMap, Request, Response, StatusCode};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::{convert::Infallible, error::Error};
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::{models, header, AuthenticationApi};

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<BoxBody<Bytes, Infallible>>, crate::ServiceError>>;

use crate::{Api,
     AnyOfGetResponse,
     CallbackWithHeaderPostResponse,
     ComplexQueryParamGetResponse,
     ExamplesTestResponse,
     FormTestResponse,
     GetWithBooleanParameterResponse,
     JsonComplexQueryParamGetResponse,
     MandatoryRequestHeaderGetResponse,
     MergePatchJsonGetResponse,
     MultigetGetResponse,
     MultipleAuthSchemeGetResponse,
     OneOfGetResponse,
     OverrideServerGetResponse,
     ParamgetGetResponse,
     ReadonlyAuthSchemeGetResponse,
     RegisterCallbackPostResponse,
     RequiredOctetStreamPutResponse,
     ResponsesWithHeadersGetResponse,
     Rfc7807GetResponse,
     TwoFirstLetterHeadersResponse,
     UntypedPropertyGetResponse,
     UuidGetResponse,
     XmlExtraPostResponse,
     XmlOtherPostResponse,
     XmlOtherPutResponse,
     XmlPostResponse,
     XmlPutResponse,
     EnumInPathPathParamGetResponse,
     MultiplePathParamsWithVeryLongPathToTestFormattingPathParamAPathParamBGetResponse,
     CreateRepoResponse,
     GetRepoInfoResponse
};

mod server_auth;

pub mod callbacks;

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/any-of$",
            r"^/callback-with-header$",
            r"^/complex-query-param$",
            r"^/enum_in_path/(?P<path_param>[^/?#]*)$",
            r"^/examples-test$",
            r"^/form-test$",
            r"^/get-with-bool$",
            r"^/json-complex-query-param$",
            r"^/mandatory-request-header$",
            r"^/merge-patch-json$",
            r"^/multiget$",
            r"^/multiple-path-params-with-very-long-path-to-test-formatting/(?P<path_param_a>[^/?#]*)/(?P<path_param_b>[^/?#]*)$",
            r"^/multiple_auth_scheme$",
            r"^/one-of$",
            r"^/operation-two-first-letter-headers$",
            r"^/override-server$",
            r"^/paramget$",
            r"^/readonly_auth_scheme$",
            r"^/register-callback$",
            r"^/repos$",
            r"^/repos/(?P<repoId>[^/?#]*)$",
            r"^/required_octet_stream$",
            r"^/responses_with_headers$",
            r"^/rfc7807$",
            r"^/untyped_property$",
            r"^/uuid$",
            r"^/xml$",
            r"^/xml_extra$",
            r"^/xml_other$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_ANY_OF: usize = 0;
    pub(crate) static ID_CALLBACK_WITH_HEADER: usize = 1;
    pub(crate) static ID_COMPLEX_QUERY_PARAM: usize = 2;
    pub(crate) static ID_ENUM_IN_PATH_PATH_PARAM: usize = 3;
    lazy_static! {
        pub static ref REGEX_ENUM_IN_PATH_PATH_PARAM: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/enum_in_path/(?P<path_param>[^/?#]*)$")
                .expect("Unable to create regex for ENUM_IN_PATH_PATH_PARAM");
    }
    pub(crate) static ID_EXAMPLES_TEST: usize = 4;
    pub(crate) static ID_FORM_TEST: usize = 5;
    pub(crate) static ID_GET_WITH_BOOL: usize = 6;
    pub(crate) static ID_JSON_COMPLEX_QUERY_PARAM: usize = 7;
    pub(crate) static ID_MANDATORY_REQUEST_HEADER: usize = 8;
    pub(crate) static ID_MERGE_PATCH_JSON: usize = 9;
    pub(crate) static ID_MULTIGET: usize = 10;
    pub(crate) static ID_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B: usize = 11;
    lazy_static! {
        pub static ref REGEX_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/multiple-path-params-with-very-long-path-to-test-formatting/(?P<path_param_a>[^/?#]*)/(?P<path_param_b>[^/?#]*)$")
                .expect("Unable to create regex for MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B");
    }
    pub(crate) static ID_MULTIPLE_AUTH_SCHEME: usize = 12;
    pub(crate) static ID_ONE_OF: usize = 13;
    pub(crate) static ID_OPERATION_TWO_FIRST_LETTER_HEADERS: usize = 14;
    pub(crate) static ID_OVERRIDE_SERVER: usize = 15;
    pub(crate) static ID_PARAMGET: usize = 16;
    pub(crate) static ID_READONLY_AUTH_SCHEME: usize = 17;
    pub(crate) static ID_REGISTER_CALLBACK: usize = 18;
    pub(crate) static ID_REPOS: usize = 19;
    pub(crate) static ID_REPOS_REPOID: usize = 20;
    lazy_static! {
        pub static ref REGEX_REPOS_REPOID: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/repos/(?P<repoId>[^/?#]*)$")
                .expect("Unable to create regex for REPOS_REPOID");
    }
    pub(crate) static ID_REQUIRED_OCTET_STREAM: usize = 21;
    pub(crate) static ID_RESPONSES_WITH_HEADERS: usize = 22;
    pub(crate) static ID_RFC7807: usize = 23;
    pub(crate) static ID_UNTYPED_PROPERTY: usize = 24;
    pub(crate) static ID_UUID: usize = 25;
    pub(crate) static ID_XML: usize = 26;
    pub(crate) static ID_XML_EXTRA: usize = 27;
    pub(crate) static ID_XML_OTHER: usize = 28;
}


pub struct MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Self {
            api_impl: self.api_impl.clone(),
            marker: PhantomData,
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, target: Target) -> Self::Future {
        let service = Service::new(self.api_impl.clone());

        future::ok(service)
    }
}

fn method_not_allowed() -> Result<Response<BoxBody<Bytes, Infallible>>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(BoxBody::new(http_body_util::Empty::new()))
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

#[allow(dead_code)]
fn body_from_string(s: String) -> BoxBody<Bytes, Infallible> {
    BoxBody::new(Full::new(Bytes::from(s)))
}

fn body_from_str(s: &str) -> BoxBody<Bytes, Infallible> {
    BoxBody::new(Full::new(Bytes::copy_from_slice(s.as_bytes())))
}

impl<T, C, ReqBody> hyper::service::Service<(Request<ReqBody>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
    ReqBody: Body + Send + 'static,
    ReqBody::Error: Into<Box<dyn Error + Send + Sync>> + Send,
    ReqBody::Data: Send,
{
    type Response = Response<BoxBody<Bytes, Infallible>>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn call(&self, req: (Request<ReqBody>, C)) -> Self::Future {
        async fn run<T, C, ReqBody>(
            mut api_impl: T,
            req: (Request<ReqBody>, C),
        ) -> Result<Response<BoxBody<Bytes, Infallible>>, crate::ServiceError>
        where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
            ReqBody: Body + Send + 'static,
            ReqBody::Error: Into<Box<dyn Error + Send + Sync>> + Send,
            ReqBody::Data: Send,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match method {

            // AnyOfGet - GET /any-of
            hyper::Method::GET if path.matched(paths::ID_ANY_OF) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_any_of = query_params.iter().filter(|e| e.0 == "any-of").map(|e| e.1.clone())
                    .filter_map(|param_any_of| param_any_of.parse().ok())
                    .collect::<Vec<_>>();
                let param_any_of = if !param_any_of.is_empty() {
                    Some(param_any_of)
                } else {
                    None
                };

                                let result = api_impl.any_of_get(
                                            param_any_of.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AnyOfGetResponse::Success
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                AnyOfGetResponse::AlternateSuccess
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                AnyOfGetResponse::AnyOfSuccess
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(202).expect("Unable to turn 202 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CallbackWithHeaderPost - POST /callback-with-header
            hyper::Method::POST if path.matched(paths::ID_CALLBACK_WITH_HEADER) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_url = query_params.iter().filter(|e| e.0 == "url").map(|e| e.1.clone())
                    .next();
                let param_url = match param_url {
                    Some(param_url) => {
                        let param_url =
                            <String as std::str::FromStr>::from_str
                                (&param_url);
                        match param_url {
                            Ok(param_url) => Some(param_url),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter url - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter url")),
                        }
                    },
                    None => None,
                };
                let param_url = match param_url {
                    Some(param_url) => param_url,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(body_from_str("Missing required query parameter url"))
                        .expect("Unable to create Bad Request response for missing query parameter url")),
                };

                                let result = api_impl.callback_with_header_post(
                                            param_url,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CallbackWithHeaderPostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ComplexQueryParamGet - GET /complex-query-param
            hyper::Method::GET if path.matched(paths::ID_COMPLEX_QUERY_PARAM) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_list_of_strings = query_params.iter().filter(|e| e.0 == "list-of-strings").map(|e| e.1.clone())
                    .filter_map(|param_list_of_strings| param_list_of_strings.parse().ok())
                    .collect::<Vec<_>>();
                let param_list_of_strings = if !param_list_of_strings.is_empty() {
                    Some(param_list_of_strings)
                } else {
                    None
                };

                                let result = api_impl.complex_query_param_get(
                                            param_list_of_strings.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ComplexQueryParamGetResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ExamplesTest - GET /examples-test
            hyper::Method::GET if path.matched(paths::ID_EXAMPLES_TEST) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_ids = query_params.iter().filter(|e| e.0 == "ids").map(|e| e.1.clone())
                    .filter_map(|param_ids| param_ids.parse().ok())
                    .collect::<Vec<_>>();
                let param_ids = if !param_ids.is_empty() {
                    Some(param_ids)
                } else {
                    None
                };

                                let result = api_impl.examples_test(
                                            param_ids.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ExamplesTestResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // FormTest - POST /form-test
            hyper::Method::POST if path.matched(paths::ID_FORM_TEST) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                // Form parameters
                                let param_required_array =
                                    None;


                                let result = api_impl.form_test(
                                            param_required_array.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FormTestResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // GetWithBooleanParameter - GET /get-with-bool
            hyper::Method::GET if path.matched(paths::ID_GET_WITH_BOOL) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_iambool = query_params.iter().filter(|e| e.0 == "iambool").map(|e| e.1.clone())
                    .next();
                let param_iambool = match param_iambool {
                    Some(param_iambool) => {
                        let param_iambool =
                            <bool as std::str::FromStr>::from_str
                                (&param_iambool);
                        match param_iambool {
                            Ok(param_iambool) => Some(param_iambool),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter iambool - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter iambool")),
                        }
                    },
                    None => None,
                };
                let param_iambool = match param_iambool {
                    Some(param_iambool) => param_iambool,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(body_from_str("Missing required query parameter iambool"))
                        .expect("Unable to create Bad Request response for missing query parameter iambool")),
                };

                                let result = api_impl.get_with_boolean_parameter(
                                            param_iambool,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetWithBooleanParameterResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // JsonComplexQueryParamGet - GET /json-complex-query-param
            hyper::Method::GET if path.matched(paths::ID_JSON_COMPLEX_QUERY_PARAM) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_list_of_strings = query_params.iter().filter(|e| e.0 == "list-of-strings").map(|e| e.1.clone())
                    .next();
                let param_list_of_strings = match param_list_of_strings {
                    Some(param_list_of_strings) => {
                        let param_list_of_strings =
                            serde_json::from_str::<Vec<models::StringObject>>
                                (&param_list_of_strings);
                        match param_list_of_strings {
                            Ok(param_list_of_strings) => Some(param_list_of_strings),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter list-of-strings - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter list-of-strings")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.json_complex_query_param_get(
                                            param_list_of_strings.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                JsonComplexQueryParamGetResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MandatoryRequestHeaderGet - GET /mandatory-request-header
            hyper::Method::GET if path.matched(paths::ID_MANDATORY_REQUEST_HEADER) => {
                // Header parameters
                let param_x_header = headers.get(HeaderName::from_static("x-header"));

                let param_x_header = match param_x_header {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Invalid header X-Header - {err}")))
                                        .expect("Unable to create Bad Request response for invalid header X-Header"));

                        },
                    },
                    None => {
                        return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_str("Missing required header X-Header"))
                                        .expect("Unable to create Bad Request response for missing required header X-Header"));
                    }
                };

                                let result = api_impl.mandatory_request_header_get(
                                            param_x_header,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MandatoryRequestHeaderGetResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MergePatchJsonGet - GET /merge-patch-json
            hyper::Method::GET if path.matched(paths::ID_MERGE_PATCH_JSON) => {
                                let result = api_impl.merge_patch_json_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MergePatchJsonGetResponse::Merge
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/merge-patch+json")
                                                            .expect("Unable to create Content-Type header for application/merge-patch+json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MultigetGet - GET /multiget
            hyper::Method::GET if path.matched(paths::ID_MULTIGET) => {
                                let result = api_impl.multiget_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MultigetGetResponse::JSONRsp
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                MultigetGetResponse::XMLRsp
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml")
                                                            .expect("Unable to create Content-Type header for application/xml"));
                                                    // XML Body
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                MultigetGetResponse::OctetRsp
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(202).expect("Unable to turn 202 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/octet-stream")
                                                            .expect("Unable to create Content-Type header for application/octet-stream"));
                                                    // Binary Body
                                                    let body = String::from_utf8(body.0).expect("Error converting octet stream to string");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                MultigetGetResponse::StringRsp
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(203).expect("Unable to turn 203 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for text/plain"));
                                                    // Plain text Body
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                MultigetGetResponse::DuplicateResponseLongText
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                MultigetGetResponse::DuplicateResponseLongText_2
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(205).expect("Unable to turn 205 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                MultigetGetResponse::DuplicateResponseLongText_3
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(206).expect("Unable to turn 206 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MultipleAuthSchemeGet - GET /multiple_auth_scheme
            hyper::Method::GET if path.matched(paths::ID_MULTIPLE_AUTH_SCHEME) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(body_from_str("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "test.read".to_string(), // Allowed to read state.
                            "test.write".to_string(), // Allowed to change state.
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(BoxBody::new(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{s} {scope}"))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                                let result = api_impl.multiple_auth_scheme_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MultipleAuthSchemeGetResponse::CheckThatLimitingToMultipleRequiredAuthSchemesWorks
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // OneOfGet - GET /one-of
            hyper::Method::GET if path.matched(paths::ID_ONE_OF) => {
                                let result = api_impl.one_of_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                OneOfGetResponse::Success
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // OverrideServerGet - GET /override-server
            hyper::Method::GET if path.matched(paths::ID_OVERRIDE_SERVER) => {
                                let result = api_impl.override_server_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                OverrideServerGetResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ParamgetGet - GET /paramget
            hyper::Method::GET if path.matched(paths::ID_PARAMGET) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_uuid = query_params.iter().filter(|e| e.0 == "uuid").map(|e| e.1.clone())
                    .next();
                let param_uuid = match param_uuid {
                    Some(param_uuid) => {
                        let param_uuid =
                            <uuid::Uuid as std::str::FromStr>::from_str
                                (&param_uuid);
                        match param_uuid {
                            Ok(param_uuid) => Some(param_uuid),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter uuid - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter uuid")),
                        }
                    },
                    None => None,
                };
                let param_some_object = query_params.iter().filter(|e| e.0 == "someObject").map(|e| e.1.clone())
                    .next();
                let param_some_object = match param_some_object {
                    Some(param_some_object) => {
                        let param_some_object =
                            <models::ObjectParam as std::str::FromStr>::from_str
                                (&param_some_object);
                        match param_some_object {
                            Ok(param_some_object) => Some(param_some_object),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter someObject - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter someObject")),
                        }
                    },
                    None => None,
                };
                let param_some_list = query_params.iter().filter(|e| e.0 == "someList").map(|e| e.1.clone())
                    .next();
                let param_some_list = match param_some_list {
                    Some(param_some_list) => {
                        let param_some_list =
                            <models::MyIdList as std::str::FromStr>::from_str
                                (&param_some_list);
                        match param_some_list {
                            Ok(param_some_list) => Some(param_some_list),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter someList - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter someList")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.paramget_get(
                                            param_uuid,
                                            param_some_object,
                                            param_some_list,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ParamgetGetResponse::JSONRsp
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ReadonlyAuthSchemeGet - GET /readonly_auth_scheme
            hyper::Method::GET if path.matched(paths::ID_READONLY_AUTH_SCHEME) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(body_from_str("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "test.read".to_string(), // Allowed to read state.
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(BoxBody::new(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{s} {scope}"))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                                let result = api_impl.readonly_auth_scheme_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ReadonlyAuthSchemeGetResponse::CheckThatLimitingToASingleRequiredAuthSchemeWorks
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // RegisterCallbackPost - POST /register-callback
            hyper::Method::POST if path.matched(paths::ID_REGISTER_CALLBACK) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_url = query_params.iter().filter(|e| e.0 == "url").map(|e| e.1.clone())
                    .next();
                let param_url = match param_url {
                    Some(param_url) => {
                        let param_url =
                            <String as std::str::FromStr>::from_str
                                (&param_url);
                        match param_url {
                            Ok(param_url) => Some(param_url),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(body_from_string(format!("Couldn't parse query parameter url - doesn't match schema: {e}")))
                                .expect("Unable to create Bad Request response for invalid query parameter url")),
                        }
                    },
                    None => None,
                };
                let param_url = match param_url {
                    Some(param_url) => param_url,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(body_from_str("Missing required query parameter url"))
                        .expect("Unable to create Bad Request response for missing query parameter url")),
                };

                                let result = api_impl.register_callback_post(
                                            param_url,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RegisterCallbackPostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // RequiredOctetStreamPut - PUT /required_octet_stream
            hyper::Method::PUT if path.matched(paths::ID_REQUIRED_OCTET_STREAM) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let param_body: Option<swagger::ByteArray> = if !body.is_empty() {
                                    Some(swagger::ByteArray(body.to_vec()))
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(BoxBody::new("Missing required body parameter body".to_string()))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };


                                let result = api_impl.required_octet_stream_put(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RequiredOctetStreamPutResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // ResponsesWithHeadersGet - GET /responses_with_headers
            hyper::Method::GET if path.matched(paths::ID_RESPONSES_WITH_HEADERS) => {
                                let result = api_impl.responses_with_headers_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ResponsesWithHeadersGetResponse::Success
                                                    {
                                                        body,
                                                        success_info,
                                                        bool_header,
                                                        object_header
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let success_info = match header::IntoHeaderValue(success_info).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(body_from_string(format!("An internal server error occurred handling success_info header - {e}")))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("success-info"),
                                                        success_info
                                                    );

                                                    if let Some(bool_header) = bool_header {
                                                    let bool_header = match header::IntoHeaderValue(bool_header).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(body_from_string(format!("An internal server error occurred handling bool_header header - {e}")))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("bool-header"),
                                                        bool_header
                                                    );
                                                    }

                                                    if let Some(object_header) = object_header {
                                                    let object_header = match header::IntoHeaderValue(object_header).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(body_from_string(format!("An internal server error occurred handling object_header header - {e}")))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("object-header"),
                                                        object_header
                                                    );
                                                    }
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                ResponsesWithHeadersGetResponse::PreconditionFailed
                                                    {
                                                        further_info,
                                                        failure_info
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(412).expect("Unable to turn 412 into a StatusCode");

                                                    if let Some(further_info) = further_info {
                                                    let further_info = match header::IntoHeaderValue(further_info).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(body_from_string(format!("An internal server error occurred handling further_info header - {e}")))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("further-info"),
                                                        further_info
                                                    );
                                                    }

                                                    if let Some(failure_info) = failure_info {
                                                    let failure_info = match header::IntoHeaderValue(failure_info).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(body_from_string(format!("An internal server error occurred handling failure_info header - {e}")))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("failure-info"),
                                                        failure_info
                                                    );
                                                    }

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // Rfc7807Get - GET /rfc7807
            hyper::Method::GET if path.matched(paths::ID_RFC7807) => {
                                let result = api_impl.rfc7807_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                Rfc7807GetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                Rfc7807GetResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for application/problem+json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                Rfc7807GetResponse::NotAcceptable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(406).expect("Unable to turn 406 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+xml")
                                                            .expect("Unable to create Content-Type header for application/problem+xml"));
                                                    // XML Body
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // TwoFirstLetterHeaders - POST /operation-two-first-letter-headers
            hyper::Method::POST if path.matched(paths::ID_OPERATION_TWO_FIRST_LETTER_HEADERS) => {
                // Header parameters
                let param_x_header_one = headers.get(HeaderName::from_static("x-header-one"));

                let param_x_header_one = match param_x_header_one {
                    Some(v) => match header::IntoHeaderValue::<bool>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Invalid header x-header-one - {err}")))
                                        .expect("Unable to create Bad Request response for invalid header x-header-one"));

                        },
                    },
                    None => {
                        None
                    }
                };
                let param_x_header_two = headers.get(HeaderName::from_static("x-header-two"));

                let param_x_header_two = match param_x_header_two {
                    Some(v) => match header::IntoHeaderValue::<bool>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Invalid header x-header-two - {err}")))
                                        .expect("Unable to create Bad Request response for invalid header x-header-two"));

                        },
                    },
                    None => {
                        None
                    }
                };

                                let result = api_impl.two_first_letter_headers(
                                            param_x_header_one,
                                            param_x_header_two,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TwoFirstLetterHeadersResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UntypedPropertyGet - GET /untyped_property
            hyper::Method::GET if path.matched(paths::ID_UNTYPED_PROPERTY) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_object_untyped_props: Option<models::ObjectUntypedProps> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    serde_ignored::deserialize(deserializer, |path| {
                                        warn!("Ignoring unknown field in body: {path}");
                                        unused_elements.push(path.to_string());
                                    }).unwrap_or_default()

                                } else {
                                    None
                                };


                                let result = api_impl.untyped_property_get(
                                            param_object_untyped_props,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                UntypedPropertyGetResponse::CheckThatUntypedPropertiesWorks
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // UuidGet - GET /uuid
            hyper::Method::GET if path.matched(paths::ID_UUID) => {
                                let result = api_impl.uuid_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UuidGetResponse::DuplicateResponseLongText
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // XmlExtraPost - POST /xml_extra
            hyper::Method::POST if path.matched(paths::ID_XML_EXTRA) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_duplicate_xml_object: Option<models::DuplicateXmlObject> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    serde_ignored::deserialize(deserializer, |path| {
                                        warn!("Ignoring unknown field in body: {path}");
                                        unused_elements.push(path.to_string());
                                    }).unwrap_or_default()

                                } else {
                                    None
                                };


                                let result = api_impl.xml_extra_post(
                                            param_duplicate_xml_object,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                XmlExtraPostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");

                                                },
                                                XmlExtraPostResponse::BadRequest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // XmlOtherPost - POST /xml_other
            hyper::Method::POST if path.matched(paths::ID_XML_OTHER) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_another_xml_object: Option<models::AnotherXmlObject> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    serde_ignored::deserialize(deserializer, |path| {
                                        warn!("Ignoring unknown field in body: {path}");
                                        unused_elements.push(path.to_string());
                                    }).unwrap_or_default()

                                } else {
                                    None
                                };


                                let result = api_impl.xml_other_post(
                                            param_another_xml_object,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                XmlOtherPostResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/xml")
                                                            .expect("Unable to create Content-Type header for text/xml"));
                                                    // XML Body
                                                    // An empty string is used to indicate a global namespace in xmltree.
                                                    let config = serde_xml_rs::SerdeXml::new()
                                                        .namespace("", models::AnotherXmlObject::NAMESPACE);
                                                    let body = config.to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                                XmlOtherPostResponse::BadRequest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // XmlOtherPut - PUT /xml_other
            hyper::Method::PUT if path.matched(paths::ID_XML_OTHER) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_another_xml_array: Option<models::AnotherXmlArray> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    serde_ignored::deserialize(deserializer, |path| {
                                        warn!("Ignoring unknown field in body: {path}");
                                        unused_elements.push(path.to_string());
                                    }).unwrap_or_default()

                                } else {
                                    None
                                };


                                let result = api_impl.xml_other_put(
                                            param_another_xml_array,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                XmlOtherPutResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");

                                                },
                                                XmlOtherPutResponse::BadRequest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // XmlPost - POST /xml
            hyper::Method::POST if path.matched(paths::ID_XML) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_xml_array: Option<models::XmlArray> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    serde_ignored::deserialize(deserializer, |path| {
                                        warn!("Ignoring unknown field in body: {path}");
                                        unused_elements.push(path.to_string());
                                    }).unwrap_or_default()

                                } else {
                                    None
                                };


                                let result = api_impl.xml_post(
                                            param_xml_array,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                XmlPostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");

                                                },
                                                XmlPostResponse::BadRequest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // XmlPut - PUT /xml
            hyper::Method::PUT if path.matched(paths::ID_XML) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_xml_object: Option<models::XmlObject> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    serde_ignored::deserialize(deserializer, |path| {
                                        warn!("Ignoring unknown field in body: {path}");
                                        unused_elements.push(path.to_string());
                                    }).unwrap_or_default()

                                } else {
                                    None
                                };


                                let result = api_impl.xml_put(
                                            param_xml_object,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                XmlPutResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");

                                                },
                                                XmlPutResponse::BadRequest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // EnumInPathPathParamGet - GET /enum_in_path/{path_param}
            hyper::Method::GET if path.matched(paths::ID_ENUM_IN_PATH_PATH_PARAM) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ENUM_IN_PATH_PATH_PARAM
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ENUM_IN_PATH_PATH_PARAM in set but failed match against \"{}\"", path, paths::REGEX_ENUM_IN_PATH_PATH_PARAM.as_str())
                    );

                let param_path_param = match percent_encoding::percent_decode(path_params["path_param"].as_bytes()).decode_utf8() {
                    Ok(param_path_param) => match param_path_param.parse::<models::StringEnum>() {
                        Ok(param_path_param) => param_path_param,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't parse path parameter path_param: {e}")))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["path_param"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.enum_in_path_path_param_get(
                                            param_path_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                EnumInPathPathParamGetResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MultiplePathParamsWithVeryLongPathToTestFormattingPathParamAPathParamBGet - GET /multiple-path-params-with-very-long-path-to-test-formatting/{path_param_a}/{path_param_b}
            hyper::Method::GET if path.matched(paths::ID_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B in set but failed match against \"{}\"", path, paths::REGEX_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B.as_str())
                    );

                let param_path_param_a = match percent_encoding::percent_decode(path_params["path_param_a"].as_bytes()).decode_utf8() {
                    Ok(param_path_param_a) => match param_path_param_a.parse::<String>() {
                        Ok(param_path_param_a) => param_path_param_a,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't parse path parameter path_param_a: {e}")))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["path_param_a"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_path_param_b = match percent_encoding::percent_decode(path_params["path_param_b"].as_bytes()).decode_utf8() {
                    Ok(param_path_param_b) => match param_path_param_b.parse::<String>() {
                        Ok(param_path_param_b) => param_path_param_b,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't parse path parameter path_param_b: {e}")))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["path_param_b"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.multiple_path_params_with_very_long_path_to_test_formatting_path_param_a_path_param_b_get(
                                            param_path_param_a,
                                            param_path_param_b,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MultiplePathParamsWithVeryLongPathToTestFormattingPathParamAPathParamBGetResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CreateRepo - POST /repos
            hyper::Method::POST if path.matched(paths::ID_REPOS) => {
                // Handle body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = http_body_util::BodyExt::collect(body).await.map(|f| f.to_bytes().to_vec());
                match result {
                     Ok(body) => {
                                let mut unused_elements : Vec<String> = vec![];
                                let param_object_param: Option<models::ObjectParam> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {path}");
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_object_param) => param_object_param,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(BoxBody::new(format!("Couldn't parse body parameter ObjectParam - doesn't match schema: {e}")))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ObjectParam due to schema")),
                                    }

                                } else {
                                    None
                                };
                                let param_object_param = match param_object_param {
                                    Some(param_object_param) => param_object_param,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(BoxBody::new("Missing required body parameter ObjectParam".to_string()))
                                                        .expect("Unable to create Bad Request response for missing body parameter ObjectParam")),
                                };


                                let result = api_impl.create_repo(
                                            param_object_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {unused_elements:?}").as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }
                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateRepoResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(body_from_string(format!("Unable to read body: {}", e.into())))
                                                .expect("Unable to create Bad Request response due to unable to read body")),
                        }
            },

            // GetRepoInfo - GET /repos/{repoId}
            hyper::Method::GET if path.matched(paths::ID_REPOS_REPOID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_REPOS_REPOID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE REPOS_REPOID in set but failed match against \"{}\"", path, paths::REGEX_REPOS_REPOID.as_str())
                    );

                let param_repo_id = match percent_encoding::percent_decode(path_params["repoId"].as_bytes()).decode_utf8() {
                    Ok(param_repo_id) => match param_repo_id.parse::<String>() {
                        Ok(param_repo_id) => param_repo_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't parse path parameter repoId: {e}")))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(body_from_string(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["repoId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_repo_info(
                                            param_repo_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(BoxBody::new(http_body_util::Empty::new()));
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetRepoInfoResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = body_from_string(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = body_from_str("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_ANY_OF) => method_not_allowed(),
            _ if path.matched(paths::ID_CALLBACK_WITH_HEADER) => method_not_allowed(),
            _ if path.matched(paths::ID_COMPLEX_QUERY_PARAM) => method_not_allowed(),
            _ if path.matched(paths::ID_ENUM_IN_PATH_PATH_PARAM) => method_not_allowed(),
            _ if path.matched(paths::ID_EXAMPLES_TEST) => method_not_allowed(),
            _ if path.matched(paths::ID_FORM_TEST) => method_not_allowed(),
            _ if path.matched(paths::ID_GET_WITH_BOOL) => method_not_allowed(),
            _ if path.matched(paths::ID_JSON_COMPLEX_QUERY_PARAM) => method_not_allowed(),
            _ if path.matched(paths::ID_MANDATORY_REQUEST_HEADER) => method_not_allowed(),
            _ if path.matched(paths::ID_MERGE_PATCH_JSON) => method_not_allowed(),
            _ if path.matched(paths::ID_MULTIGET) => method_not_allowed(),
            _ if path.matched(paths::ID_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B) => method_not_allowed(),
            _ if path.matched(paths::ID_MULTIPLE_AUTH_SCHEME) => method_not_allowed(),
            _ if path.matched(paths::ID_ONE_OF) => method_not_allowed(),
            _ if path.matched(paths::ID_OPERATION_TWO_FIRST_LETTER_HEADERS) => method_not_allowed(),
            _ if path.matched(paths::ID_OVERRIDE_SERVER) => method_not_allowed(),
            _ if path.matched(paths::ID_PARAMGET) => method_not_allowed(),
            _ if path.matched(paths::ID_READONLY_AUTH_SCHEME) => method_not_allowed(),
            _ if path.matched(paths::ID_REGISTER_CALLBACK) => method_not_allowed(),
            _ if path.matched(paths::ID_REPOS) => method_not_allowed(),
            _ if path.matched(paths::ID_REPOS_REPOID) => method_not_allowed(),
            _ if path.matched(paths::ID_REQUIRED_OCTET_STREAM) => method_not_allowed(),
            _ if path.matched(paths::ID_RESPONSES_WITH_HEADERS) => method_not_allowed(),
            _ if path.matched(paths::ID_RFC7807) => method_not_allowed(),
            _ if path.matched(paths::ID_UNTYPED_PROPERTY) => method_not_allowed(),
            _ if path.matched(paths::ID_UUID) => method_not_allowed(),
            _ if path.matched(paths::ID_XML) => method_not_allowed(),
            _ if path.matched(paths::ID_XML_EXTRA) => method_not_allowed(),
            _ if path.matched(paths::ID_XML_OTHER) => method_not_allowed(),
                _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                        .body(BoxBody::new(http_body_util::Empty::new()))
                        .expect("Unable to create Not Found response"))
            }
        }
        Box::pin(run(
            self.api_impl.clone(),
            req,
        ))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // AnyOfGet - GET /any-of
            hyper::Method::GET if path.matched(paths::ID_ANY_OF) => Some("AnyOfGet"),
            // CallbackWithHeaderPost - POST /callback-with-header
            hyper::Method::POST if path.matched(paths::ID_CALLBACK_WITH_HEADER) => Some("CallbackWithHeaderPost"),
            // ComplexQueryParamGet - GET /complex-query-param
            hyper::Method::GET if path.matched(paths::ID_COMPLEX_QUERY_PARAM) => Some("ComplexQueryParamGet"),
            // ExamplesTest - GET /examples-test
            hyper::Method::GET if path.matched(paths::ID_EXAMPLES_TEST) => Some("ExamplesTest"),
            // FormTest - POST /form-test
            hyper::Method::POST if path.matched(paths::ID_FORM_TEST) => Some("FormTest"),
            // GetWithBooleanParameter - GET /get-with-bool
            hyper::Method::GET if path.matched(paths::ID_GET_WITH_BOOL) => Some("GetWithBooleanParameter"),
            // JsonComplexQueryParamGet - GET /json-complex-query-param
            hyper::Method::GET if path.matched(paths::ID_JSON_COMPLEX_QUERY_PARAM) => Some("JsonComplexQueryParamGet"),
            // MandatoryRequestHeaderGet - GET /mandatory-request-header
            hyper::Method::GET if path.matched(paths::ID_MANDATORY_REQUEST_HEADER) => Some("MandatoryRequestHeaderGet"),
            // MergePatchJsonGet - GET /merge-patch-json
            hyper::Method::GET if path.matched(paths::ID_MERGE_PATCH_JSON) => Some("MergePatchJsonGet"),
            // MultigetGet - GET /multiget
            hyper::Method::GET if path.matched(paths::ID_MULTIGET) => Some("MultigetGet"),
            // MultipleAuthSchemeGet - GET /multiple_auth_scheme
            hyper::Method::GET if path.matched(paths::ID_MULTIPLE_AUTH_SCHEME) => Some("MultipleAuthSchemeGet"),
            // OneOfGet - GET /one-of
            hyper::Method::GET if path.matched(paths::ID_ONE_OF) => Some("OneOfGet"),
            // OverrideServerGet - GET /override-server
            hyper::Method::GET if path.matched(paths::ID_OVERRIDE_SERVER) => Some("OverrideServerGet"),
            // ParamgetGet - GET /paramget
            hyper::Method::GET if path.matched(paths::ID_PARAMGET) => Some("ParamgetGet"),
            // ReadonlyAuthSchemeGet - GET /readonly_auth_scheme
            hyper::Method::GET if path.matched(paths::ID_READONLY_AUTH_SCHEME) => Some("ReadonlyAuthSchemeGet"),
            // RegisterCallbackPost - POST /register-callback
            hyper::Method::POST if path.matched(paths::ID_REGISTER_CALLBACK) => Some("RegisterCallbackPost"),
            // RequiredOctetStreamPut - PUT /required_octet_stream
            hyper::Method::PUT if path.matched(paths::ID_REQUIRED_OCTET_STREAM) => Some("RequiredOctetStreamPut"),
            // ResponsesWithHeadersGet - GET /responses_with_headers
            hyper::Method::GET if path.matched(paths::ID_RESPONSES_WITH_HEADERS) => Some("ResponsesWithHeadersGet"),
            // Rfc7807Get - GET /rfc7807
            hyper::Method::GET if path.matched(paths::ID_RFC7807) => Some("Rfc7807Get"),
            // TwoFirstLetterHeaders - POST /operation-two-first-letter-headers
            hyper::Method::POST if path.matched(paths::ID_OPERATION_TWO_FIRST_LETTER_HEADERS) => Some("TwoFirstLetterHeaders"),
            // UntypedPropertyGet - GET /untyped_property
            hyper::Method::GET if path.matched(paths::ID_UNTYPED_PROPERTY) => Some("UntypedPropertyGet"),
            // UuidGet - GET /uuid
            hyper::Method::GET if path.matched(paths::ID_UUID) => Some("UuidGet"),
            // XmlExtraPost - POST /xml_extra
            hyper::Method::POST if path.matched(paths::ID_XML_EXTRA) => Some("XmlExtraPost"),
            // XmlOtherPost - POST /xml_other
            hyper::Method::POST if path.matched(paths::ID_XML_OTHER) => Some("XmlOtherPost"),
            // XmlOtherPut - PUT /xml_other
            hyper::Method::PUT if path.matched(paths::ID_XML_OTHER) => Some("XmlOtherPut"),
            // XmlPost - POST /xml
            hyper::Method::POST if path.matched(paths::ID_XML) => Some("XmlPost"),
            // XmlPut - PUT /xml
            hyper::Method::PUT if path.matched(paths::ID_XML) => Some("XmlPut"),
            // EnumInPathPathParamGet - GET /enum_in_path/{path_param}
            hyper::Method::GET if path.matched(paths::ID_ENUM_IN_PATH_PATH_PARAM) => Some("EnumInPathPathParamGet"),
            // MultiplePathParamsWithVeryLongPathToTestFormattingPathParamAPathParamBGet - GET /multiple-path-params-with-very-long-path-to-test-formatting/{path_param_a}/{path_param_b}
            hyper::Method::GET if path.matched(paths::ID_MULTIPLE_PATH_PARAMS_WITH_VERY_LONG_PATH_TO_TEST_FORMATTING_PATH_PARAM_A_PATH_PARAM_B) => Some("MultiplePathParamsWithVeryLongPathToTestFormattingPathParamAPathParamBGet"),
            // CreateRepo - POST /repos
            hyper::Method::POST if path.matched(paths::ID_REPOS) => Some("CreateRepo"),
            // GetRepoInfo - GET /repos/{repoId}
            hyper::Method::GET if path.matched(paths::ID_REPOS_REPOID) => Some("GetRepoInfo"),
            _ => None,
        }
    }
}
