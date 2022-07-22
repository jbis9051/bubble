# Setting up Mapbox for iOS

## React-Native > `0.60.0`

Please make sure that `@rnmapbox/maps` is installed via `yarn`

---

## Creating Secret Access Token

In order to authorize the Mapbox API, a Mapbox secret access token with the `DOWNLOADS:READ` scope will need to be created. 

Create the secret access token by creating a mapbox account using the following link:
https://account.mapbox.com

Once the secret access token has been created, store it in the `.netrc` file. The `.netrc` file is stored in the home directory if it has already been created.

Paste the following at the bottom of the file:

```
machine api.mapbox.com
login mapbox
password YOUR_SECRET_MAPBOX_ACCESS_TOKEN
```
Replace `YOUR_SECRET_MAPBOX_ACCESS_TOKEN` with the secret token.

---
## Creating Public Token

In order to set the access token for MapboxGL, a public token must be created. Go to https://account.mapbox.com to create the public token. Activate all of the public scopes and create the token. 

Create an environment variable called `REACT_APP_MAPBOX_ACCESS_TOKEN` and set it equal to the public token created.

---
## Building and Running


To ensure that everything has been installed, run the following commands:
```bash
# Go to the ios folder
cd ios

# Run Pod Install
pod install
```

Go to the `Map.tsx` file in `packages/js/app/src/components/Profile/Map.tsx` file and replace the `MapView` component with the `MapboxGL.MapView` component for iOS in order to test Mapbox on iOS devices. 

Go to `packages/js/app` and run `yarn ios`.
