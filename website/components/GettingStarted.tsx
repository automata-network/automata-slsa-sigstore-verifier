import React from 'react';
import CodeBlock from './ui/CodeBlock';
import { Shield, Download, Key, Globe, Lock } from 'lucide-react';

const GettingStarted: React.FC = () => {
  return (
    <section id="bundles" className="py-16 md:py-24 border-b border-zinc-900 bg-black overflow-hidden">
      <div className="max-w-7xl mx-auto px-4 md:px-6">

        <div className="mb-12 md:mb-16">
          <h2 className="text-2xl md:text-4xl font-bold text-white mb-4 md:mb-6">Getting Started: Bundles & Trust Roots</h2>
          <p className="text-zinc-400 max-w-3xl leading-relaxed">
            Generate attestations with GitHub Actions, download bundles with GitHub CLI, and fetch trust roots for verification.
            Our implementation supports both public and private repository workflows.
          </p>
        </div>

        <div className="space-y-12 md:space-y-20">

          {/* Step 1 */}
          <div className="grid lg:grid-cols-12 gap-6 md:gap-12">
            <div className="lg:col-span-4">
              <div className="flex items-center gap-3 mb-4 text-orange-500">
                <Shield className="w-5 h-5 md:w-6 md:h-6 flex-shrink-0" />
                <h3 className="text-lg md:text-xl font-bold text-white">Step 1: Generate Attestations</h3>
              </div>
              <p className="text-zinc-400 mb-6">
                Use GitHub Actions to automatically generate Sigstore attestations during your build process.
              </p>
            </div>
            <div className="lg:col-span-8">
              <CodeBlock 
                language="yaml" 
                title="GitHub Actions Workflow"
                code={`name: Build and Attest
on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      attestations: write
      id-token: write

    steps:
      - uses: actions/checkout@v5

      - name: Build artifact
        run: |
          # Your build commands
          echo "built" > artifact.txt

      - name: Attest build provenance
        uses: actions/attest-build-provenance@v3
        with:
          subject-path: artifact.txt`}
              />
            </div>
          </div>

          {/* Step 2 */}
          <div className="grid lg:grid-cols-12 gap-6 md:gap-12">
            <div className="lg:col-span-4">
              <div className="flex items-center gap-3 mb-4 text-blue-500">
                <Download className="w-5 h-5 md:w-6 md:h-6 flex-shrink-0" />
                <h3 className="text-lg md:text-xl font-bold text-white">Step 2: Download Bundles</h3>
              </div>
              <p className="text-zinc-400 mb-6">
                After GitHub Actions generates the attestation, download the Sigstore bundle using the GitHub CLI.
              </p>
              
              {/* Bundle Types Sub-section */}
              <div className="mt-8 space-y-4">
                <h4 className="text-sm font-mono text-zinc-500 uppercase tracking-wider">Bundle Types</h4>
                <p className="text-xs text-zinc-500 mb-2">GitHub generates different bundles depending on repository visibility.</p>
                <div className="p-4 border border-zinc-800 bg-zinc-900/30">
                  <div className="flex items-center gap-2 mb-3 text-emerald-500">
                    <Globe size={16} />
                    <span className="font-bold text-sm uppercase">Public Repos</span>
                  </div>
                  <p className="text-xs text-zinc-500 mb-2">Public Good Instance</p>
                  <ul className="text-xs text-zinc-400 space-y-2">
                    <li><span className="text-zinc-300">Fulcio Issuer:</span> Public Good Sigstore CA</li>
                    <li><span className="text-zinc-300">Timestamping:</span> Rekor Transparency Log with Merkle inclusion proof</li>
                    <li><span className="text-zinc-300">Inclusion Proof:</span> Cryptographic proof that the cert exists in the public log</li>
                  </ul>
                </div>
                <div className="p-4 border border-zinc-800 bg-zinc-900/30">
                  <div className="flex items-center gap-2 mb-3 text-orange-500">
                    <Lock size={16} />
                    <span className="font-bold text-sm uppercase">Private Repos</span>
                  </div>
                  <p className="text-xs text-zinc-500 mb-2">GitHub Instance</p>
                  <ul className="text-xs text-zinc-400 space-y-2">
                    <li><span className="text-zinc-300">Fulcio Issuer:</span> GitHub Internal Services Root</li>
                    <li><span className="text-zinc-300">Timestamping:</span> RFC 3161 Timestamp Authority (TSA) with certificate chain</li>
                    <li><span className="text-zinc-300">Privacy:</span> Signed Timestamp is kept private; not posted to public logs</li>
                  </ul>
                </div>
              </div>

            </div>
            <div className="lg:col-span-8 space-y-4">
              <CodeBlock
                language="bash"
                title="Using local artifact path"
                code={`gh attestation download local/path/to/artifact -R OWNER/REPO`}
              />
              <CodeBlock
                language="bash"
                title="Using container image URI"
                code={`gh attestation download oci://<image-uri> -R OWNER/REPO`}
              />
              <CodeBlock
                language="bash"
                title="Using CURL directly from GitHub"
                code={`curl https://github.com/OWNER/REPO/attestations/<attestation-id>/download > bundle.json`}
              />
            </div>
          </div>

          {/* Step 3 */}
          <div className="grid lg:grid-cols-12 gap-6 md:gap-12">
            <div className="lg:col-span-4">
              <div className="flex items-center gap-3 mb-4 text-orange-500">
                <Key className="w-5 h-5 md:w-6 md:h-6 flex-shrink-0" />
                <h3 className="text-lg md:text-xl font-bold text-white">Step 3: Get Trust Roots</h3>
              </div>
              <p className="text-zinc-400 mb-6">
                Trust roots contain the Fulcio CA certificates and Timestamp Authority certificates needed to verify the attestation. 
                <span className="block mt-2 text-zinc-500 text-sm">Recommended: Refresh every 60-90 days.</span>
              </p>
            </div>
            <div className="lg:col-span-8">
              <CodeBlock 
                language="bash" 
                title="Fetch Trust Roots"
                code={`gh attestation trusted-root > trusted_root.jsonl`}
              />
            </div>
          </div>

        </div>
      </div>
    </section>
  );
};

export default GettingStarted;
