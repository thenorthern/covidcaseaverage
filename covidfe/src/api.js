import { call, select } from "redux-saga/effects";

const API_ROOT = "http://127.0.0.1:8088/";

export function* getApiDataFor(path: string) {
  const url = API_ROOT.concat(path);
  const options = {
    headers: new Headers({
      Accept: "application/json",
      "Content-type": "application/json",
    }),
    method: "GET",
  };
  return yield call(request, url, options);
}

function checkStatus(response) {
  if (response.status >= 200 && response.status < 300) {
    return response;
  }
  switch (response.status) {
    case 500:
    case 404:
      const error = {
        message: response.statusText,
      };
      throw error;
  }

  return new Promise((_, reject) => {
    response.text().then((text) => {
      const error = {
        message: text,
      };
      reject(error);
    });
  });
  // error.response = response;
}

function parseJSON(response) {
  if (response.status === 204 || response.status === 205) {
    return null;
  }
  const contentType = response.headers.get("content-type");
  if (contentType && contentType.indexOf("application/json") !== -1) {
    return response.json();
  }
  return response;
}

export default function request(url, options) {
  return fetch(url, options)
    .then(checkStatus)
    .then(parseJSON)
    .then((data) => data);
}
