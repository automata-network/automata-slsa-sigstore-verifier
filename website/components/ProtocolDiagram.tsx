import React from 'react';
import { User, FileSignature, ShieldCheck, Database, Key } from 'lucide-react';

const ProtocolDiagram: React.FC = () => {
  return (
    <div className="w-full bg-slate-900 rounded-xl p-8 border border-slate-800 shadow-2xl overflow-hidden relative">
      <h3 className="text-xl font-bold text-slate-200 mb-8 text-center">Sigstore Protocol Flow</h3>
      
      <div className="flex flex-col md:flex-row justify-between items-center relative z-10 gap-8 md:gap-0">
        
        {/* Developer */}
        <div className="flex flex-col items-center group cursor-pointer">
          <div className="bg-blue-600 p-4 rounded-full shadow-lg shadow-blue-900/50 mb-4 transition-transform group-hover:scale-110">
            <User className="w-8 h-8 text-white" />
          </div>
          <p className="font-semibold text-blue-200">Developer</p>
          <span className="text-xs text-slate-400 mt-1">Initiates Build</span>
        </div>

        {/* Arrow 1 */}
        <div className="hidden md:flex flex-1 h-1 bg-slate-700 mx-4 relative items-center justify-center">
            <div className="absolute -top-6 text-xs text-slate-400">OIDC Identity (GitHub Actions token)</div>
            <div className="w-2 h-2 bg-slate-500 rounded-full animate-pulse absolute right-0"></div>
        </div>

        {/* Fulcio */}
        <div className="flex flex-col items-center group cursor-pointer">
          <div className="bg-purple-600 p-4 rounded-full shadow-lg shadow-purple-900/50 mb-4 transition-transform group-hover:scale-110">
            <Key className="w-8 h-8 text-white" />
          </div>
          <p className="font-semibold text-purple-200">Fulcio</p>
          <span className="text-xs text-slate-400 mt-1">Certificate Authority</span>
        </div>

        {/* Arrow 2 */}
        <div className="hidden md:flex flex-1 h-1 bg-slate-700 mx-4 relative items-center justify-center">
             <div className="absolute -top-6 text-xs text-slate-400">Short-lived Cert (valid 10-20 min)</div>
             <div className="w-2 h-2 bg-slate-500 rounded-full animate-pulse absolute right-0"></div>
        </div>

        {/* Rekor/TSA */}
        <div className="flex flex-col items-center group cursor-pointer">
          <div className="bg-emerald-600 p-4 rounded-full shadow-lg shadow-emerald-900/50 mb-4 transition-transform group-hover:scale-110">
            <Database className="w-8 h-8 text-white" />
          </div>
          <p className="font-semibold text-emerald-200">Rekor OR TSA</p>
          <span className="text-xs text-slate-400 mt-1">Timestamp Proof</span>
        </div>

         {/* Arrow 3 */}
         <div className="hidden md:flex flex-1 h-1 bg-slate-700 mx-4 relative items-center justify-center">
             <div className="absolute -top-6 text-xs text-slate-400">Timestamp Proof (Rekor or RFC3161)</div>
             <div className="w-2 h-2 bg-slate-500 rounded-full animate-pulse absolute right-0"></div>
        </div>

        {/* Verifier */}
        <div className="flex flex-col items-center group cursor-pointer">
          <div className="bg-orange-600 p-4 rounded-full shadow-lg shadow-orange-900/50 mb-4 transition-transform group-hover:scale-110">
            <ShieldCheck className="w-8 h-8 text-white" />
          </div>
          <p className="font-semibold text-orange-200">Verifier</p>
          <span className="text-xs text-slate-400 mt-1">Validates Integrity</span>
        </div>
      </div>

      {/* Connection Lines for Mobile */}
      <div className="absolute top-1/2 left-10 right-10 h-0.5 bg-slate-800 -z-0 hidden md:block"></div>
      
      <div className="mt-12 grid grid-cols-1 md:grid-cols-2 gap-6 text-sm text-slate-400 bg-slate-950/50 p-6 rounded-lg border border-slate-800">
        <div>
            <h4 className="text-white font-semibold mb-2 flex items-center gap-2">
                <FileSignature size={16}/> Motivation
            </h4>
            <p>
                Traditional signing keys are hard to manage and often leaked. Sigstore eliminates key management by using ephemeral keys tied to OIDC identities (GitHub Actions, email),
                automatically signing artifacts during CI/CD without storing long-lived secrets.
            </p>
        </div>
        <div>
            <h4 className="text-white font-semibold mb-2 flex items-center gap-2">
                <ShieldCheck size={16}/> Why Care?
            </h4>
            <p>
                It prevents supply chain attacks and enables <strong>Proof of Provenance</strong> for on-chain verification.
                You can cryptographically prove <em>who</em> built the software, <em>when</em>, and <em>from which repository</em>,
                ensuring artifacts haven't been tampered with since the build action.
            </p>
        </div>
      </div>
    </div>
  );
};

export default ProtocolDiagram;
