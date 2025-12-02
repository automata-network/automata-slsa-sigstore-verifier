import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="py-16 md:py-20 border-t border-zinc-800 bg-zinc-950 text-center">
      <div className="flex items-center justify-center gap-3 text-zinc-300 text-base md:text-lg">
        <span>Built by</span>
        <a
          href="https://x.com/automatanetwork"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:opacity-80 transition-opacity"
        >
          <img
            src={import.meta.env.BASE_URL + 'automata-logo.svg'}
            alt="Automata"
            className="h-7 md:h-8 inline-block"
          />
        </a>
      </div>
    </footer>
  );
};

export default Footer;
