module.exports = async (phase, { defaultConfig }) => {
    /**
     * @type {import('next').NextConfig}
     */
    const nextConfig = {
        reactStrictMode: true,
        swcMinify: true,
        images: {
            unoptimized: true,
        },
        output: "export"
    }
    return nextConfig
}