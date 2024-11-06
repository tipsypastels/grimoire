// @ts-check

import { XMLBuilder, XMLParser } from "fast-xml-parser";
import * as fs from "node:fs/promises";

const dir = `${import.meta.dirname}/../fa`;
const parser = new XMLParser({ ignoreAttributes: false });
const builder = new XMLBuilder({ ignoreAttributes: false });

/**
 * @param {string} filename
 */
async function fix(filename) {
  const path = `${dir}/${filename}`;
  const icon = filename.replace(/\.svg$/, "");
  const text = await fs.readFile(path, "utf-8");
  const xml = parser.parse(text);
  const { svg } = xml;

  if (svg["@_data-icon"]) {
    console.log(`"${icon}" already fixed.`);
    return;
  }

  Object.assign(svg, {
    "@_class": `svg-inline--fa fa-${icon}`,
    "@_aria-hidden": "true",
    "@_focusable": "false",
    "@_data-prefix": "fas",
    "@_data-icon": icon,
    "@_role": "img",
  });

  const out = builder.build(xml);
  await fs.writeFile(path, out);
  console.log(`"${icon}" fixed.`);
}

/** @type {Promise<void>[]} */
const promises = [];
const files = await fs.readdir(dir);

for (const file of files) {
  promises.push(fix(file));
}

await Promise.all(promises);
