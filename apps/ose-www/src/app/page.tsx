import { Header } from "@/features/app-shell/shell/header";
import { Footer } from "@/features/app-shell/shell/footer";
import { Hero } from "@/features/landing/shell/hero";
import { SocialIcons } from "@/features/landing/shell/social-icons";

export default function Home() {
  return (
    <>
      <Header />
      <main>
        <Hero />
        <SocialIcons />
      </main>
      <Footer />
    </>
  );
}
