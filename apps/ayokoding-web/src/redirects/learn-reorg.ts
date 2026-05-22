export const learnReorgRedirects: Array<{
  source: string;
  destination: string;
  permanent: boolean;
}> = [
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
