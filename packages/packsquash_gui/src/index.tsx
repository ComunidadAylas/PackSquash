/* @refresh reload */

import "./index.css";

import { ErrorBoundary, render } from "solid-js/web";
import { hashIntegration, Route, Router, Routes } from "@solidjs/router";
import { lazy } from "solid-js";
import { MetaProvider } from "@solidjs/meta";
import { I18nProvider, Locale } from "./contexts/i18n";
import { Toaster } from "solid-toast";
import syncWindowTitleWithDocumentTitle from "./util/syncWindowTitleWithDocumentTitle";
import disableContextMenuForNonTextSelection from "./util/disableContextMenuForNonTextSelection";
import FrontendCrashScreen from "./views/FrontendCrashScreen";
import { getSelectedLocale } from "./util/localeSettings";
import { registerVisualDragAndDropPreventionHandlers } from "./util/dragAndDrop";
import konamiCode from "./util/konamiCode";
import UpdateChecker from "./components/UpdateChecker";

syncWindowTitleWithDocumentTitle();
disableContextMenuForNonTextSelection();
registerVisualDragAndDropPreventionHandlers();
konamiCode();

let selectedLocale: Locale | undefined;
try {
  selectedLocale = await getSelectedLocale();
} catch {
  // Fallback to no selected locale
}

let appElement;
if (!(appElement = document.getElementsByTagName("main")[0])) {
  throw new Error("No app root element");
}

// From now on we want to manage the page title with MetaProvider,
// but MetaProvider can't replace our no-JS friendly title. See:
// https://github.com/solidjs/solid-meta/issues/8
document.getElementById("app-title")?.remove();

const Home = lazy(() => import("./views/Home"));
const About = lazy(() => import("./views/About"));
const PackSelection = lazy(() => import("./views/PackSelection"));
const Configuration = lazy(() => import("./views/Configuration"));
const OptimizationProgress = lazy(() => import("./views/OptimizationProgress"));
const OptimizationSuccess = lazy(() => import("./views/OptimizationSuccess"));
const OptimizationFailure = lazy(() => import("./views/OptimizationFailure"));

render(
  () => (
    <I18nProvider selectedLocale={selectedLocale}>
      <MetaProvider>
        <ErrorBoundary
          fallback={(err, reset) => (
            <FrontendCrashScreen throwable={err} reset={reset} />
          )}
        >
          <Toaster position="bottom-right" />
          <UpdateChecker />
          <Router source={hashIntegration()}>
            <Routes>
              <Route path="/" component={Home} />
              <Route path="/about" component={About} />
              <Route path="/packSelection" component={PackSelection} />
              <Route path="/configuration" component={Configuration} />
              <Route
                path="/optimizationProgress"
                component={OptimizationProgress}
              />
              <Route
                path="/optimizationSuccess"
                component={OptimizationSuccess}
              />
              <Route
                path="/optimizationFailure"
                component={OptimizationFailure}
              />
            </Routes>
          </Router>
        </ErrorBoundary>
      </MetaProvider>
    </I18nProvider>
  ),
  appElement
);
