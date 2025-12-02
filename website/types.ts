export interface NavLink {
  label: string;
  href: string;
}

export interface CodeSnippet {
  language: 'bash' | 'yaml' | 'json' | 'rust' | 'solidity';
  code: string;
  title?: string;
}

export enum TabOption {
  INPUTS = 'INPUTS',
  PROCESS = 'ZK PROCESS',
  OUTPUTS = 'OUTPUTS'
}
