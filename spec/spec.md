# Echo Server Spec

This is an overview spec document which details the endpoint and what they expect to receive and in turn what responses
you can expect.

## Security

The details of Echo Server's Security are in [auth.md](./auth.md)

## Responses

These are the default responses and are only different if otherwise specified.

### Successful Response

```
{
    "status": "OK"
}
```

### Unsuccessful Response

```
{
    "status": "FAILED",
    "reasons": [
        { "field": "type", "description": "", "location": "" }
    ]
}
```

> **Note** `location` should be treated as an enum (`body`, `query`, `header`) where `body` is the
> default unless otherwise specified.

## Requests

### Health

```
GET <ECHO_SERVER_URL>/health
```

#### Example Response

```
OK, echo-server v0.1.0
```

### Register Client

```
POST <ECHO_SERVER_URL>/clients
{
    "client_id": <CLIENT_ID>,
    "type": <TYPE>,
    "token": <DEVICE_TOKEN>
}
```

- `CLIENT_ID`: The Client's ID from the Relay pairing.
- `TYPE`: The push service to use e.g. APNS, FCM. This will be validated against the supported types on the Echo
  Server's side.
- `DEVICE_TOKEN`: The device's token for the push service e.g. FCM, APNS.

### Unregister Client

```
DELETE <ECHO_SERVER_URL>/clients/<CLIENT_ID>
```

- `CLIENT_ID`: The Client's ID from the Relay pairing.

### Send Notification

> **Note**
> This will have E2E encryption in the future but for now is designed without encryption.

```
POST <ECHO_SERVER_URL>/clients/<CLIENT_ID>
{
    "id": "0000-0000-0000-0000"
    "payload": {
        "title": "New Request",
        "description": "You have a signing request."
    }
}
```

- `CLIENT_ID`: This is used to map to the device token
- `id`: This is generated by the Relay to prevent duplicate notifications
- `payload`: A JSON object that holds the relevant payload data. **NOTE:** This will become a string that is encrypted
  by the dApp and not decrypted until received by the client