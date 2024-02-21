export type Html = string;

export interface PageModel {
  route: string;
  title: string;
  description: string;
  part: string | null;
  outline: OutlineItem[];
  body: BodyModel;
  children: PageModel[];
}

export interface OutlineItem {
  id: string;
  name: string;
  children: OutlineItem[];
}

export type BodyModel =
  | { kind: "Html"; content: Html }
  | { kind: "Category"; content: CategoryModel }
  | { kind: "Func"; content: FuncModel }
  | { kind: "Group"; content: GroupModel }
  | { kind: "Type"; content: TypeModel }
  | { kind: "Symbols"; content: SymbolsModel }
  | { kind: "Packages"; content: Html };

export interface CategoryModel {
  name: string;
  title: string;
  details: Html;
  items: CategoryItem[];
  shorthands: ShorthandsModel | null;
}

export interface CategoryItem {
  name: string;
  route: string;
  oneliner: string;
  code: boolean;
}

export interface FuncModel {
  path: string[];
  name: string;
  title: string;
  keywords: string[];
  oneliner: string;
  element: boolean;
  details: Html;
  example: Html | null;
  self_: boolean;
  params: ParamModel[];
  returns: string[];
  scope: FuncModel[];
}

export interface ParamModel {
  name: string;
  details: Html;
  example: Html | null;
  types: string[];
  strings: StrParam[];
  default: Html | null;
  positional: boolean;
  named: boolean;
  required: boolean;
  variadic: boolean;
  settable: boolean;
}

export interface StrParam {
  string: string;
  details: Html;
}

export interface GroupModel {
  name: string;
  title: string;
  details: Html;
  functions: FuncModel[];
}

export interface TypeModel {
  name: string;
  title: string;
  keywords: string[];
  oneliner: string;
  details: Html;
  constructor: FuncModel | null;
  scope: FuncModel[];
}

export interface SymbolsModel {
  name: string;
  title: string;
  details: Html;
  list: SymbolModel[];
}

export interface SymbolModel {
  name: string;
  codepoint: number;
  accent: boolean;
  unicodeName: string | null;
  alternates: string[];
  markupShorthand: string | null;
  mathShorthand: string | null;
}

export interface ShorthandsModel {
  markup: SymbolModel[];
  math: SymbolModel[];
}
