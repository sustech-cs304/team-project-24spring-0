/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export',
    distDir: process.env.NODE_ENV === 'release' ? 'out' : 'tmp/' + process.env.R,
}

export default nextConfig
