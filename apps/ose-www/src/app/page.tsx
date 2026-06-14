import { Header } from "@/features/app-shell/presentation/header";
import { Footer } from "@/features/app-shell/presentation/footer";
import { Hero } from "@/features/landing/presentation/hero";
import { SocialIcons } from "@/features/landing/presentation/social-icons";

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
