declare global {
  var Stimulus: Application;
}

import "@hotwired/turbo";
import { Application } from "@hotwired/stimulus";
import { TestController } from "../controllers/test";

window.Stimulus ??= Application.start();

Stimulus.register("test", TestController);
