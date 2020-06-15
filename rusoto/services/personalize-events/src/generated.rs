// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Represents user interaction event information sent using the <code>PutEvents</code> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Event {
    /// <p>An ID associated with the event. If an event ID is not provided, Amazon Personalize generates a unique ID for the event. An event ID is not used as an input to the model. Amazon Personalize uses the event ID to distinquish unique events. Any subsequent events after the first with the same event ID are not used in model training.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>The type of event. This property corresponds to the <code>EVENT_TYPE</code> field of the Interactions schema.</p>
    #[serde(rename = "eventType")]
    pub event_type: String,
    /// <p>A string map of event-specific data that you might choose to record. For example, if a user rates a movie on your site, you might send the movie ID and rating, and the number of movie ratings made by the user.</p> <p>Each item in the map consists of a key-value pair. For example,</p> <p> <code>{"itemId": "movie1"}</code> </p> <p> <code>{"itemId": "movie2", "eventValue": "4.5"}</code> </p> <p> <code>{"itemId": "movie3", "eventValue": "3", "numberOfRatings": "12"}</code> </p> <p>The keys use camel case names that match the fields in the Interactions schema. The <code>itemId</code> and <code>eventValue</code> keys correspond to the <code>ITEM_ID</code> and <code>EVENT_VALUE</code> fields. In the above example, the <code>eventType</code> might be 'MovieRating' with <code>eventValue</code> being the rating. The <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Interactions schema.</p>
    #[serde(rename = "properties")]
    pub properties: String,
    /// <p>The timestamp on the client side when the event occurred.</p>
    #[serde(rename = "sentAt")]
    pub sent_at: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequest {
    /// <p>A list of event data from the session.</p>
    #[serde(rename = "eventList")]
    pub event_list: Vec<Event>,
    /// <p>The session ID associated with the user's visit.</p>
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
    /// <p>The user associated with the event.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
}

impl PutEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(PutEventsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventsError {}
/// Trait representing the capabilities of the Amazon Personalize Events API. Amazon Personalize Events clients implement this trait.
#[async_trait]
pub trait PersonalizeEvents {
    /// <p>Records user interaction event data.</p>
    async fn put_events(&self, input: PutEventsRequest) -> Result<(), RusotoError<PutEventsError>>;
}
/// A client for the Amazon Personalize Events API.
#[derive(Clone)]
pub struct PersonalizeEventsClient {
    client: Client,
    region: region::Region,
}

impl PersonalizeEventsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PersonalizeEventsClient {
        PersonalizeEventsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PersonalizeEventsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PersonalizeEventsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PersonalizeEventsClient {
        PersonalizeEventsClient { client, region }
    }
}

#[async_trait]
impl PersonalizeEvents for PersonalizeEventsClient {
    /// <p>Records user interaction event data.</p>
    #[allow(unused_mut)]
    async fn put_events(&self, input: PutEventsRequest) -> Result<(), RusotoError<PutEventsError>> {
        let request_uri = "/events";

        let mut request = SignedRequest::new("POST", "personalize", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("personalize-events".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventsError::from_response(response))
        }
    }
}
