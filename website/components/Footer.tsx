import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="py-12 border-t border-zinc-900 bg-zinc-950 text-center">
      <div className="flex items-center justify-center gap-2 text-zinc-500 text-sm">
        <span>Built by</span>
        <a
          href="https://x.com/automatanetwork"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:opacity-80 transition-opacity"
        >
          <img
            src="/automata-logo.svg"
            alt="Automata"
            className="h-5 inline-block"
          />
        </a>
      </div>
    </footer>
  );
};

export default Footer;
