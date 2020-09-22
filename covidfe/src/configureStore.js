import { createStore, applyMiddleware, compose } from "redux";
import rootReducer from "./rootReducer";
import reduxImmutableStateInvariant from "redux-immutable-state-invariant";
import reduxSaga from "redux-saga";
import sagas from "./sagas";

export default function configureStore(initialState) {
  const sagaMiddleware = reduxSaga();
  const middlewares = applyMiddleware(
    sagaMiddleware,
    reduxImmutableStateInvariant(),
  );
  const composeEnhancers =
    window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose; // add support for Redux dev tools
  const enhancer = composeEnhancers(middlewares);

  let store = createStore(rootReducer, initialState, enhancer);

  sagaMiddleware.run(sagas);

  return store;
}

// const sagaMiddleware = reduxSaga();

// const middlewares = applyMiddleware(sagaMiddleware, launchDarkly);
// const composeEnhancers =
//   (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
// const enhancer = composeEnhancers(middlewares);
// const store = createStore<IAppState>(reducers, enhancer);

// sagaMiddleware.run(sagas);
