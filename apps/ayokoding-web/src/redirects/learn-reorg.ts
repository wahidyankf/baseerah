export const learnReorgRedirects: Array<{
  source: string;
  destination: string;
  permanent: boolean;
}> = [
  // Phase 7 — cases → by-example/cases
  {
    source: "/en/learn/software-engineering/software-architecture/cases/:path*",
    destination: "/en/learn/software-engineering/software-architecture/by-example/cases/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/software-engineering/system-design/cases/:path*",
    destination: "/en/learn/software-engineering/system-design/by-example/cases/:path*",
    permanent: true,
  },
  // Phase 6 — infrastructure concepts fold-in
  {
    source: "/en/learn/software-engineering/infrastructure/concepts/how-to/:path*",
    destination: "/en/learn/software-engineering/infrastructure/by-example/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/software-engineering/infrastructure/concepts/:path*",
    destination: "/en/learn/software-engineering/infrastructure/by-concept/:path*",
    permanent: true,
  },
  // Phase 5 — information-security track normalization
  {
    source: "/en/learn/information-security/concepts/explanation/:path*",
    destination: "/en/learn/information-security/by-concept/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/information-security/foundations/by-example/:path*",
    destination: "/en/learn/information-security/by-example/foundations/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/information-security/foundations/:path*",
    destination: "/en/learn/information-security/by-concept/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/information-security/concepts/:path*",
    destination: "/en/learn/information-security/by-concept/:path*",
    permanent: true,
  },
  // Phase 4 — human → personal-development
  {
    source: "/en/learn/human/:path*",
    destination: "/en/learn/personal-development/:path*",
    permanent: true,
  },
  // Phase 3 — algorithm plural rename
  {
    source: "/en/learn/software-engineering/algorithm-and-data-structures/:path*",
    destination: "/en/learn/software-engineering/algorithms-and-data-structures/:path*",
    permanent: true,
  },
  // Phase 2 — platforms rename
  {
    source: "/en/learn/software-engineering/platform-linux/:path*",
    destination: "/en/learn/software-engineering/platforms/linux/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/software-engineering/platform-web/:path*",
    destination: "/en/learn/software-engineering/platforms/web/:path*",
    permanent: true,
  },
  {
    source: "/en/learn/software-engineering/platform-mobile/:path*",
    destination: "/en/learn/software-engineering/platforms/mobile/:path*",
    permanent: true,
  },
];
