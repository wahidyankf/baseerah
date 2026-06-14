import { redirect } from "next/navigation";

// The marketing landing page now lives in the dedicated `organiclever-www`
// site. The app client serves the application directly: the root route
// redirects to the app home.
export default function RootPage() {
  redirect("/app/home");
}
