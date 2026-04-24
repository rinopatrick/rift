<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { EditorView, keymap, lineNumbers, highlightActiveLineGutter, highlightSpecialChars, drawSelection, dropCursor, rectangularSelection, crosshairCursor, highlightActiveLine } from "@codemirror/view";
  import { EditorState, type Extension } from "@codemirror/state";
  import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
  import { bracketMatching, syntaxHighlighting, foldGutter, foldKeymap, indentOnInput, StreamLanguage } from "@codemirror/language";
  import { sql } from "@codemirror/lang-sql";
  import { autocompletion, completionKeymap, closeBrackets, closeBracketsKeymap, type Completion } from "@codemirror/autocomplete";
  import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
  import { lintKeymap } from "@codemirror/lint";
  import { HighlightStyle, syntaxHighlighting as syntaxHighlightingExt, type TagStyle } from "@codemirror/language";
  import { tags as t } from "@lezer/highlight";

  interface Props {
    value: string;
    onChange: (value: string) => void;
    onRun: () => void;
    onRunNewTab: () => void;
    placeholder?: string;
    disabled?: boolean;
    completions?: Completion[];
  }

  let { value, onChange, onRun, onRunNewTab, placeholder = "-- Write your SQL here", disabled = false, completions = [] }: Props = $props();

  let editorEl: HTMLDivElement;
  let view: EditorView;

  const riftTheme = EditorView.theme({
    "&": {
      backgroundColor: "#0c0c0c",
      color: "#e8e8e8",
      fontSize: "13px",
      fontFamily: '"JetBrains Mono", "Fira Code", "Source Code Pro", Menlo, Monaco, monospace',
      height: "100%",
    },
    ".cm-content": {
      caretColor: "#00d4ff",
      padding: "12px 0",
      lineHeight: "1.6",
    },
    "&.cm-focused .cm-cursor": {
      borderLeftColor: "#00d4ff",
      borderLeftWidth: "2px",
    },
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground": {
      backgroundColor: "#1a3a4a",
    },
    ".cm-gutters": {
      backgroundColor: "#0c0c0c",
      color: "#4a4a4a",
      borderRight: "1px solid #1a1a1a",
      paddingLeft: "8px",
      paddingRight: "8px",
    },
    ".cm-activeLineGutter": {
      backgroundColor: "#141414",
      color: "#6b6b6b",
    },
    ".cm-line": {
      padding: "0 12px",
    },
    ".cm-activeLine": {
      backgroundColor: "#141414",
    },
    ".cm-matchingBracket": {
      backgroundColor: "#1a3a4a",
      outline: "1px solid #00d4ff33",
    },
    ".cm-tooltip": {
      backgroundColor: "#1a1a1a",
      border: "1px solid #2a2a2a",
      borderRadius: "6px",
      boxShadow: "0 4px 12px rgba(0,0,0,0.5)",
      color: "#e8e8e8",
    },
    ".cm-tooltip.cm-tooltip-autocomplete": {
      "& > ul": {
        backgroundColor: "#1a1a1a",
        borderRadius: "6px",
        maxHeight: "280px",
        fontFamily: '"JetBrains Mono", monospace',
        fontSize: "12px",
      },
      "& > ul > li": {
        padding: "6px 12px",
        borderRadius: "4px",
      },
      "& > ul > li[aria-selected]": {
        backgroundColor: "#00d4ff22",
        color: "#00d4ff",
      },
    },
    ".cm-completionLabel": {
      color: "#e8e8e8",
    },
    ".cm-completionDetail": {
      color: "#6b6b6b",
      fontSize: "11px",
      marginLeft: "8px",
    },
    ".cm-completionIcon": {
      display: "none",
    },
    ".cm-placeholder": {
      color: "#4a4a4a",
      fontStyle: "italic",
    },
    ".cm-panels": {
      backgroundColor: "#141414",
      color: "#e8e8e8",
      borderTop: "1px solid #2a2a2a",
    },
    ".cm-searchMatch": {
      backgroundColor: "#00d4ff33",
      outline: "1px solid #00d4ff66",
    },
    ".cm-searchMatch.cm-searchMatch-selected": {
      backgroundColor: "#00d4ff66",
    },
    ".cm-foldPlaceholder": {
      backgroundColor: "#1a1a1a",
      border: "1px solid #2a2a2a",
      color: "#6b6b6b",
      borderRadius: "4px",
      padding: "0 4px",
    },
  }, { dark: true });

  const riftHighlightStyle = HighlightStyle.define([
    { tag: t.keyword, color: "#00d4ff", fontWeight: "bold" },
    { tag: [t.name, t.deleted, t.character, t.propertyName, t.macroName], color: "#e8e8e8" },
    { tag: [t.function(t.variableName), t.labelName], color: "#82b1ff" },
    { tag: [t.color, t.constant(t.name), t.standard(t.name)], color: "#ffab91" },
    { tag: [t.definition(t.name), t.separator], color: "#e8e8e8" },
    { tag: [t.typeName, t.className, t.number, t.changed, t.annotation, t.modifier, t.self, t.namespace], color: "#ce93d8" },
    { tag: [t.operator, t.operatorKeyword, t.url, t.escape, t.regexp, t.link, t.special(t.string)], color: "#89ddff" },
    { tag: [t.meta, t.comment], color: "#6b6b6b", fontStyle: "italic" },
    { tag: t.strong, fontWeight: "bold", color: "#e8e8e8" },
    { tag: t.emphasis, fontStyle: "italic", color: "#e8e8e8" },
    { tag: t.strikethrough, textDecoration: "line-through" },
    { tag: t.link, color: "#82b1ff", textDecoration: "underline" },
    { tag: [t.string, t.processingInstruction, t.punctuation], color: "#a5d6a7" },
    { tag: t.invalid, color: "#ff5370", borderBottom: "1px dotted #ff5370" },
    { tag: t.bool, color: "#ffab91" },
    { tag: t.atom, color: "#ce93d8" },
  ]);

  function createSchemaCompletions(items: Completion[]) {
    return (context: any) => {
      const word = context.matchBefore(/[\w.]*/);
      if (!word || (word.from === word.to && !context.explicit)) return null;
      
      const filter = word.text.toLowerCase();
      const options = items.filter(item => 
        item.label.toLowerCase().includes(filter) || 
        (item.detail && item.detail.toLowerCase().includes(filter))
      );
      
      if (options.length === 0) return null;
      
      return {
        from: word.from,
        options: options.slice(0, 50),
      };
    };
  }

  function createExtensions(): Extension[] {
    const ext: Extension[] = [
      riftTheme,
      syntaxHighlightingExt(riftHighlightStyle),
      lineNumbers(),
      highlightActiveLineGutter(),
      highlightSpecialChars(),
      history(),
      foldGutter({
        markerDOM: (open) => {
          const el = document.createElement("span");
          el.textContent = open ? "\u25bc" : "\u25b6";
          el.style.color = "#4a4a4a";
          el.style.fontSize = "10px";
          return el;
        },
      }),
      drawSelection(),
      dropCursor(),
      EditorState.allowMultipleSelections.of(true),
      indentOnInput(),
      bracketMatching(),
      closeBrackets(),
      autocompletion({
        override: completions.length > 0 ? [createSchemaCompletions(completions)] : [],
        defaultKeymap: true,
        closeOnBlur: true,
        maxRenderedOptions: 30,
      }),
      rectangularSelection(),
      crosshairCursor(),
      highlightActiveLine(),
      highlightSelectionMatches(),
      sql(),
      keymap.of([
        indentWithTab,
        ...closeBracketsKeymap,
        ...defaultKeymap,
        ...searchKeymap,
        ...historyKeymap,
        ...foldKeymap,
        ...completionKeymap,
        ...lintKeymap,
        {
          key: "Ctrl-Enter",
          run: () => {
            onRun();
            return true;
          },
          preventDefault: true,
        },
        {
          key: "Ctrl-Shift-Enter",
          run: () => {
            onRunNewTab();
            return true;
          },
          preventDefault: true,
        },
      ]),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          onChange(update.state.doc.toString());
        }
      }),
      EditorView.editable.of(!disabled),
    ];
    return ext;
  }

  onMount(() => {
    const state = EditorState.create({
      doc: value,
      extensions: createExtensions(),
    });
    view = new EditorView({ state, parent: editorEl });
  });

  onDestroy(() => {
    view?.destroy();
  });

  $effect(() => {
    if (view && value !== view.state.doc.toString()) {
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: value },
        scrollIntoView: false,
      });
    }
  });

  $effect(() => {
    if (view) {
      view.dispatch({ effects: EditorView.reconfigure.of(createExtensions()) });
    }
  });
</script>

<div bind:this={editorEl} class="flex-1 overflow-hidden"></div>

<style>
  :global(.cm-editor) {
    height: 100%;
  }
  :global(.cm-scroller) {
    overflow: auto;
  }
</style>
