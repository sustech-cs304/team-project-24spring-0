/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  distDir: 'tmp/' + process.env.R,
}

export default nextConfig
