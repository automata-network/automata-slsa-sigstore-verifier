import React, { useRef, useEffect } from 'react';

/**
 * A procedural aesthetic component that renders a rotating cube
 * represented by a "halftone" field of dots.
 * 
 * Logic:
 * 1. Generate a 3D grid of points.
 * 2. Project points to 2D screen space.
 * 3. Calculate "lighting" based on normal vectors (simplified to position for abstract look).
 * 4. Render circles (dots) where radius = light intensity.
 */

const HalftoneMonolith: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let animationFrameId: number;
    let time = 0;

    // Configuration
    const gridSize = 14; // Number of dots per axis
    const spacing = 18; // Space between dots
    const cubeSize = (gridSize - 1) * spacing;
    const offset = cubeSize / 2;

    interface Point3D {
      x: number;
      y: number;
      z: number;
      ox: number; // original x
      oy: number;
      oz: number;
    }

    // Generate Grid
    const points: Point3D[] = [];
    for (let x = 0; x < gridSize; x++) {
      for (let y = 0; y < gridSize; y++) {
        for (let z = 0; z < gridSize; z++) {
          points.push({
            x: x * spacing - offset,
            y: y * spacing - offset,
            z: z * spacing - offset,
            ox: x * spacing - offset,
            oy: y * spacing - offset,
            oz: z * spacing - offset,
          });
        }
      }
    }

    const render = () => {
      time += 0.005;
      
      // Handle resize
      if (canvas.width !== canvas.offsetWidth || canvas.height !== canvas.offsetHeight) {
        canvas.width = canvas.offsetWidth;
        canvas.height = canvas.offsetHeight;
      }
      
      const width = canvas.width;
      const height = canvas.height;
      const cx = width / 2;
      const cy = height / 2;

      ctx.clearRect(0, 0, width, height);
      
      // Background subtle grain is handled by CSS, we just draw the object here.
      // Rotation Matrices
      const cosX = Math.cos(time * 0.5);
      const sinX = Math.sin(time * 0.5);
      const cosY = Math.cos(time * 0.8);
      const sinY = Math.sin(time * 0.8);

      // Light Source Vector (Static relative to camera)
      const lx = 0.5;
      const ly = -1;
      const lz = -0.5;
      const lLen = Math.sqrt(lx*lx + ly*ly + lz*lz);

      const projectedPoints: { x: number, y: number, z: number, size: number }[] = [];

      for (let i = 0; i < points.length; i++) {
        const p = points[i];

        // Rotate Y
        let rx = p.ox * cosY - p.oz * sinY;
        let rz = p.ox * sinY + p.oz * cosY;
        // Rotate X
        let ry = p.oy * cosX - rz * sinX;
        rz = p.oy * sinX + rz * cosX;

        // Simple perspective projection
        const fov = 600;
        const scale = fov / (fov + rz + 400); // +400 moves it back
        const x2d = rx * scale + cx;
        const y2d = ry * scale + cy;

        // Halftone / Dithering Logic
        // Calculate "brightness" based on position/normal approximation
        // Dots closer to the light source (top-left-front) are larger
        // We simulate a normal by using the point's position relative to center
        
        // Normalize rotated position
        const mag = Math.sqrt(rx*rx + ry*ry + rz*rz) || 1;
        const nx = rx / mag;
        const ny = ry / mag;
        const nz = rz / mag;

        // Dot Product with light
        const dot = (nx * lx + ny * ly + nz * lz) / lLen;
        
        // Map dot product (-1 to 1) to radius (0.5 to 3.5)
        // We want strict contrast for the "dithered" look
        let radius = Math.max(0, (dot + 0.6) * 3);
        
        // Distance fade
        const depthAlpha = Math.max(0.1, (1 - (rz + offset) / (cubeSize * 2)));

        if (radius > 0.1) {
            projectedPoints.push({
                x: x2d,
                y: y2d,
                z: rz,
                size: radius * depthAlpha * scale
            });
        }
      }

      // Sort by Z (Painter's algorithm) to handle occlusion if we were drawing solids,
      // but for dots, z-sort helps visual consistency.
      projectedPoints.sort((a, b) => b.z - a.z);

      ctx.fillStyle = '#ffffff'; // Monochrome white dots
      
      for (let i = 0; i < projectedPoints.length; i++) {
        const p = projectedPoints[i];
        ctx.beginPath();
        // Draw square dots for a more digital/pixelated dither look, or circles for traditional halftone
        // Using circles for "Classic Halftone"
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fill();
      }

      animationFrameId = requestAnimationFrame(render);
    };

    render();

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return (
    <div className="w-full h-full flex items-center justify-center overflow-hidden grayscale contrast-125">
      <canvas 
        ref={canvasRef} 
        className="w-full h-[600px] md:h-[800px] max-w-[800px] opacity-90"
      />
    </div>
  );
};

export default HalftoneMonolith;
