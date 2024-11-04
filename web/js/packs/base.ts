declare global {
  var Stimulus: import("@hotwired/stimulus").Application;
}

import { Application } from "@hotwired/stimulus";
import { TestController } from "../controllers/test";

window.Stimulus ??= Application.start();

Stimulus.register("test", TestController);