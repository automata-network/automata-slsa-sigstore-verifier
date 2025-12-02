import React, { useState, useMemo } from 'react';
import { Check, Copy } from 'lucide-react';

interface CodeBlockProps {
  code: string;
  language: string;
  title?: string;
}

// Orange-themed syntax highlighting colors
const colors = {
  keyword: 'text-orange-400',        // Keywords, control flow
  string: 'text-amber-300',          // Strings
  comment: 'text-zinc-500',          // Comments
  number: 'text-orange-300',         // Numbers
  function: 'text-amber-400',        // Functions, methods
  type: 'text-orange-500',           // Types, classes
  property: 'text-amber-200',        // Properties, keys
  operator: 'text-zinc-400',         // Operators
  variable: 'text-zinc-200',         // Variables
  punctuation: 'text-zinc-500',      // Brackets, punctuation
  builtin: 'text-orange-400',        // Built-in functions
  attribute: 'text-amber-500',       // Attributes, decorators
};

type TokenType = keyof typeof colors;

interface Token {
  type: TokenType | 'plain';
  content: string;
}

const highlightYaml = (code: string): Token[][] => {
  return code.split('\n').map(line => {
    const tokens: Token[] = [];

    // Comments
    if (line.trim().startsWith('#')) {
      return [{ type: 'comment', content: line }];
    }

    // Key-value pairs
    const keyMatch = line.match(/^(\s*)([a-zA-Z_-]+)(:)/);
    if (keyMatch) {
      const [, indent, key, colon] = keyMatch;
      tokens.push({ type: 'plain', content: indent });
      tokens.push({ type: 'property', content: key });
      tokens.push({ type: 'punctuation', content: colon });

      const rest = line.slice(keyMatch[0].length);
      if (rest.trim()) {
        // Check for strings
        if (rest.includes('"') || rest.includes("'")) {
          tokens.push({ type: 'string', content: rest });
        } else if (rest.trim().match(/^(true|false|null|yes|no)$/i)) {
          tokens.push({ type: 'keyword', content: rest });
        } else if (rest.trim().match(/^\d+$/)) {
          tokens.push({ type: 'number', content: rest });
        } else {
          tokens.push({ type: 'plain', content: rest });
        }
      }
      return tokens;
    }

    // List items
    if (line.trim().startsWith('-')) {
      const dashMatch = line.match(/^(\s*)(-)(\s*)(.*)/);
      if (dashMatch) {
        const [, indent, dash, space, content] = dashMatch;
        tokens.push({ type: 'plain', content: indent });
        tokens.push({ type: 'punctuation', content: dash });
        tokens.push({ type: 'plain', content: space });
        if (content.includes(':')) {
          const [key, ...vals] = content.split(':');
          tokens.push({ type: 'property', content: key });
          tokens.push({ type: 'punctuation', content: ':' });
          tokens.push({ type: 'plain', content: vals.join(':') });
        } else {
          tokens.push({ type: 'plain', content: content });
        }
        return tokens;
      }
    }

    return [{ type: 'plain', content: line }];
  });
};

const highlightBash = (code: string): Token[][] => {
  return code.split('\n').map(line => {
    const tokens: Token[] = [];

    // Comments
    if (line.trim().startsWith('#')) {
      return [{ type: 'comment', content: line }];
    }

    // Process the line
    let remaining = line;
    let match: RegExpMatchArray | null;
    let lastWasSpace = true; // Track if previous char was space (start of line counts)

    while (remaining.length > 0) {
      // Commands at start or after pipes/semicolons
      if ((match = remaining.match(/^(cargo|git|gh|curl|npm|echo|cd|mkdir|forge|run|build|install|test)\b/))) {
        tokens.push({ type: 'builtin', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = false;
      }
      // Flags - only highlight if preceded by space
      else if (lastWasSpace && (match = remaining.match(/^(--?[a-zA-Z][-a-zA-Z0-9]*)/))) {
        tokens.push({ type: 'keyword', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = false;
      }
      // Strings
      else if ((match = remaining.match(/^("[^"]*"|'[^']*')/))) {
        tokens.push({ type: 'string', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = false;
      }
      // Variables
      else if ((match = remaining.match(/^(\$[a-zA-Z_][a-zA-Z0-9_]*|\$\{[^}]+\})/))) {
        tokens.push({ type: 'variable', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = false;
      }
      // URLs
      else if ((match = remaining.match(/^(https?:\/\/[^\s]+)/))) {
        tokens.push({ type: 'string', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = false;
      }
      // Operators
      else if ((match = remaining.match(/^([|>&;])/))) {
        tokens.push({ type: 'operator', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = false;
      }
      // Whitespace
      else if ((match = remaining.match(/^(\s+)/))) {
        tokens.push({ type: 'plain', content: match[0] });
        remaining = remaining.slice(match[0].length);
        lastWasSpace = true;
      }
      // Plain text
      else {
        const nextSpecial = remaining.search(/[\s|>&;$"'#]/);
        if (nextSpecial === -1) {
          tokens.push({ type: 'plain', content: remaining });
          remaining = '';
        } else if (nextSpecial === 0) {
          tokens.push({ type: 'plain', content: remaining[0] });
          remaining = remaining.slice(1);
        } else {
          tokens.push({ type: 'plain', content: remaining.slice(0, nextSpecial) });
          remaining = remaining.slice(nextSpecial);
        }
        lastWasSpace = false;
      }
    }

    return tokens;
  });
};

const highlightJson = (code: string): Token[][] => {
  return code.split('\n').map(line => {
    const tokens: Token[] = [];
    let remaining = line;
    let match: RegExpMatchArray | null;

    while (remaining.length > 0) {
      // Property keys
      if ((match = remaining.match(/^(\s*)"([^"]+)"(\s*:)/))) {
        tokens.push({ type: 'plain', content: match[1] });
        tokens.push({ type: 'punctuation', content: '"' });
        tokens.push({ type: 'property', content: match[2] });
        tokens.push({ type: 'punctuation', content: '"' });
        tokens.push({ type: 'plain', content: match[3].slice(0, -1) });
        tokens.push({ type: 'punctuation', content: ':' });
        remaining = remaining.slice(match[0].length);
      }
      // Strings
      else if ((match = remaining.match(/^"([^"]*)"/))) {
        tokens.push({ type: 'string', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Numbers
      else if ((match = remaining.match(/^-?\d+\.?\d*/))) {
        tokens.push({ type: 'number', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Booleans and null
      else if ((match = remaining.match(/^(true|false|null)\b/))) {
        tokens.push({ type: 'keyword', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Comments (non-standard but often used)
      else if ((match = remaining.match(/^\/\/.*/))) {
        tokens.push({ type: 'comment', content: match[0] });
        remaining = '';
      }
      // Brackets and punctuation
      else if ((match = remaining.match(/^[{}\[\],]/))) {
        tokens.push({ type: 'punctuation', content: match[0] });
        remaining = remaining.slice(1);
      }
      // Ellipsis for truncated content
      else if ((match = remaining.match(/^\.\.\./))) {
        tokens.push({ type: 'comment', content: match[0] });
        remaining = remaining.slice(3);
      }
      else {
        tokens.push({ type: 'plain', content: remaining[0] });
        remaining = remaining.slice(1);
      }
    }

    return tokens;
  });
};

const highlightRust = (code: string): Token[][] => {
  const keywords = ['use', 'fn', 'let', 'mut', 'if', 'else', 'match', 'for', 'while', 'loop', 'return', 'pub', 'mod', 'struct', 'enum', 'impl', 'trait', 'where', 'self', 'Self', 'ref', 'const', 'static', 'type', 'async', 'await', 'move', 'dyn', 'unsafe', 'extern', 'crate'];
  const types = ['String', 'Vec', 'Option', 'Result', 'Some', 'None', 'Ok', 'Err', 'Path', 'PathBuf', 'Box', 'Rc', 'Arc', 'bool', 'i32', 'i64', 'u32', 'u64', 'f32', 'f64', 'usize', 'str'];
  const builtins = ['println', 'print', 'format', 'panic', 'assert', 'expect', 'unwrap'];

  return code.split('\n').map(line => {
    const tokens: Token[] = [];

    // Full line comments
    if (line.trim().startsWith('//')) {
      return [{ type: 'comment', content: line }];
    }

    let remaining = line;
    let match: RegExpMatchArray | null;

    while (remaining.length > 0) {
      // Inline comments - everything after // is a comment
      if ((match = remaining.match(/^\/\/.*/))) {
        tokens.push({ type: 'comment', content: match[0] });
        remaining = '';
      }
      // Macros
      else if ((match = remaining.match(/^([a-zA-Z_][a-zA-Z0-9_]*)!/))) {
        tokens.push({ type: 'builtin', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Attributes
      else if ((match = remaining.match(/^#\[([^\]]+)\]/))) {
        tokens.push({ type: 'attribute', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Strings
      else if ((match = remaining.match(/^"([^"\\]|\\.)*"/))) {
        tokens.push({ type: 'string', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Lifetime
      else if ((match = remaining.match(/^'[a-zA-Z_][a-zA-Z0-9_]*/))) {
        tokens.push({ type: 'attribute', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Keywords
      else if ((match = remaining.match(new RegExp(`^(${keywords.join('|')})\\b`)))) {
        tokens.push({ type: 'keyword', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Types
      else if ((match = remaining.match(new RegExp(`^(${types.join('|')})\\b`)))) {
        tokens.push({ type: 'type', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Type-like (PascalCase)
      else if ((match = remaining.match(/^[A-Z][a-zA-Z0-9_]*/))) {
        tokens.push({ type: 'type', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Function calls
      else if ((match = remaining.match(/^([a-z_][a-zA-Z0-9_]*)(\s*\()/))) {
        const funcName = match[1];
        if (builtins.includes(funcName)) {
          tokens.push({ type: 'builtin', content: funcName });
        } else {
          tokens.push({ type: 'function', content: funcName });
        }
        tokens.push({ type: 'punctuation', content: match[2] });
        remaining = remaining.slice(match[0].length);
      }
      // Numbers
      else if ((match = remaining.match(/^\d+\.?\d*/))) {
        tokens.push({ type: 'number', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Operators
      else if ((match = remaining.match(/^(::|\->|=>|&&|\|\||[+\-*/%=<>!&|^~?])/))) {
        tokens.push({ type: 'operator', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Punctuation
      else if ((match = remaining.match(/^[{}\[\]();,.:]/))) {
        tokens.push({ type: 'punctuation', content: match[0] });
        remaining = remaining.slice(1);
      }
      else {
        tokens.push({ type: 'plain', content: remaining[0] });
        remaining = remaining.slice(1);
      }
    }

    return tokens;
  });
};

const highlightSolidity = (code: string): Token[][] => {
  const keywords = ['pragma', 'solidity', 'import', 'from', 'contract', 'library', 'interface', 'function', 'modifier', 'event', 'struct', 'enum', 'mapping', 'public', 'private', 'internal', 'external', 'view', 'pure', 'payable', 'memory', 'storage', 'calldata', 'returns', 'return', 'if', 'else', 'for', 'while', 'do', 'break', 'continue', 'new', 'delete', 'emit', 'require', 'assert', 'revert', 'using', 'is', 'virtual', 'override', 'constructor'];
  const types = ['address', 'bool', 'string', 'bytes', 'bytes32', 'bytes4', 'uint', 'uint8', 'uint64', 'uint256', 'int', 'int256'];

  return code.split('\n').map(line => {
    const tokens: Token[] = [];

    // Comments
    if (line.trim().startsWith('//')) {
      return [{ type: 'comment', content: line }];
    }

    let remaining = line;
    let match: RegExpMatchArray | null;

    while (remaining.length > 0) {
      // Inline comments
      if ((match = remaining.match(/^\/\/.*/))) {
        tokens.push({ type: 'comment', content: match[0] });
        remaining = '';
      }
      // Strings
      else if ((match = remaining.match(/^"([^"\\]|\\.)*"/))) {
        tokens.push({ type: 'string', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Version pragma
      else if ((match = remaining.match(/^\^?\d+\.\d+\.\d+/))) {
        tokens.push({ type: 'number', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Keywords
      else if ((match = remaining.match(new RegExp(`^(${keywords.join('|')})\\b`)))) {
        tokens.push({ type: 'keyword', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Types
      else if ((match = remaining.match(new RegExp(`^(${types.join('|')})\\b`)))) {
        tokens.push({ type: 'type', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Contract/Type names (PascalCase) - but not camelCase
      else if ((match = remaining.match(/^[A-Z][a-zA-Z0-9_]*/))) {
        tokens.push({ type: 'type', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Function calls (only when followed by parenthesis)
      else if ((match = remaining.match(/^([a-z_][a-zA-Z0-9_]*)(\s*\()/))) {
        tokens.push({ type: 'function', content: match[1] });
        tokens.push({ type: 'punctuation', content: match[2] });
        remaining = remaining.slice(match[0].length);
      }
      // camelCase identifiers (variables) - keep as plain/white
      else if ((match = remaining.match(/^[a-z_][a-zA-Z0-9_]*/))) {
        tokens.push({ type: 'plain', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Numbers
      else if ((match = remaining.match(/^\d+/))) {
        tokens.push({ type: 'number', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Operators
      else if ((match = remaining.match(/^(==|!=|<=|>=|&&|\|\||[+\-*/%=<>!&|^~])/))) {
        tokens.push({ type: 'operator', content: match[0] });
        remaining = remaining.slice(match[0].length);
      }
      // Punctuation
      else if ((match = remaining.match(/^[{}\[\]();,.:]/))) {
        tokens.push({ type: 'punctuation', content: match[0] });
        remaining = remaining.slice(1);
      }
      else {
        tokens.push({ type: 'plain', content: remaining[0] });
        remaining = remaining.slice(1);
      }
    }

    return tokens;
  });
};

const highlightToml = (code: string): Token[][] => {
  return code.split('\n').map(line => {
    const tokens: Token[] = [];

    // Comments
    if (line.trim().startsWith('#')) {
      return [{ type: 'comment', content: line }];
    }

    // Section headers
    if (line.trim().startsWith('[')) {
      const match = line.match(/^(\s*)(\[+)([^\]]+)(\]+)/);
      if (match) {
        tokens.push({ type: 'plain', content: match[1] });
        tokens.push({ type: 'punctuation', content: match[2] });
        tokens.push({ type: 'type', content: match[3] });
        tokens.push({ type: 'punctuation', content: match[4] });
        return tokens;
      }
    }

    // Key-value pairs
    const kvMatch = line.match(/^(\s*)([a-zA-Z_-][a-zA-Z0-9_-]*)(\s*=\s*)(.*)/);
    if (kvMatch) {
      const [, indent, key, equals, value] = kvMatch;
      tokens.push({ type: 'plain', content: indent });
      tokens.push({ type: 'property', content: key });
      tokens.push({ type: 'operator', content: equals });

      // Parse value
      if (value.startsWith('"') || value.startsWith("'")) {
        tokens.push({ type: 'string', content: value });
      } else if (value.startsWith('{')) {
        tokens.push({ type: 'punctuation', content: value });
      } else if (value.match(/^(true|false)$/)) {
        tokens.push({ type: 'keyword', content: value });
      } else if (value.match(/^\d/)) {
        tokens.push({ type: 'number', content: value });
      } else {
        tokens.push({ type: 'plain', content: value });
      }
      return tokens;
    }

    return [{ type: 'plain', content: line }];
  });
};

const highlightCode = (code: string, language: string): Token[][] => {
  switch (language.toLowerCase()) {
    case 'yaml':
    case 'yml':
      return highlightYaml(code);
    case 'bash':
    case 'sh':
    case 'shell':
      return highlightBash(code);
    case 'json':
      return highlightJson(code);
    case 'rust':
    case 'rs':
      return highlightRust(code);
    case 'solidity':
    case 'sol':
      return highlightSolidity(code);
    case 'toml':
      return highlightToml(code);
    default:
      return code.split('\n').map(line => [{ type: 'plain', content: line }]);
  }
};

const CodeBlock: React.FC<CodeBlockProps> = ({ code, language, title }) => {
  const [copied, setCopied] = useState(false);

  const highlightedLines = useMemo(() => highlightCode(code.trim(), language), [code, language]);

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const renderToken = (token: Token, index: number) => {
    if (token.type === 'plain') {
      return <span key={index}>{token.content}</span>;
    }
    return (
      <span key={index} className={colors[token.type]}>
        {token.content}
      </span>
    );
  };

  return (
    <div className="w-full my-4 border border-zinc-800 bg-zinc-950/50 backdrop-blur-sm group overflow-hidden">
      {(title || language) && (
        <div className="flex items-center justify-between px-4 py-2 border-b border-zinc-800 bg-zinc-900/30">
          <span className="text-xs font-mono text-zinc-400 uppercase tracking-wider">
            {title || language}
          </span>
          <button
            onClick={handleCopy}
            className="text-zinc-500 hover:text-white transition-colors"
          >
            {copied ? <Check size={14} className="text-emerald-500" /> : <Copy size={14} />}
          </button>
        </div>
      )}
      <div className="p-4 overflow-x-auto">
        <pre className="font-mono text-sm leading-relaxed">
          <code>
            {highlightedLines.map((lineTokens, lineIndex) => (
              <div key={lineIndex} className="whitespace-pre">
                {lineTokens.map((token, tokenIndex) => renderToken(token, tokenIndex))}
              </div>
            ))}
          </code>
        </pre>
      </div>
    </div>
  );
};

export default CodeBlock;
