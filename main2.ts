#!/usr/bin/env -S deno run -A
import process from "node:process";
import * as YAML from "npm:yaml";

function throw_(error: any): never {
  throw error;
}

const DOC_LANG =
  process.env.DOC_LANG ||
  throw_(new ReferenceError("DOC_LANG env var not set"));

  import fs from 'fs';
import path from 'path';
import { renderFile } from 'ejs';
import yaml from 'js-yaml';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const readFile = promisify(fs.readFile);
const writeFile = promisify(fs.writeFile);
const mkdir = promisify(fs.mkdir);
const copyFile = promisify(fs.copyFile);
const copyTree = promisify(fs.copyFile);

function str_presenter(dumper, data) {
    if (data.split('\n').length > 1) {
        return dumper.representScalar('tag:yaml.org,2002:str', data, '||');
    }
    return dumper.representScalar('tag:yaml.org,2002:str', data);
}

yaml.addReplacer(str_presenter);

function translate_with_yaml(page) {
    // ... (same as in Python)
}

const type2class_map = {
    // ... (same as in Python)
};

function type2class(type) {
    return type2class_map[type] || 'pill-obj';
}

function gen_path(item) {
    return item.path.join('.');
}

async function render_jinja_html(templateLoc, fileName, context) {
    const templatePath = path.join(templateLoc, fileName);
    const template = await readFile(templatePath, 'utf-8');
    return renderFile(template, context);
}
