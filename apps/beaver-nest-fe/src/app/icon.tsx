import { ImageResponse } from "next/og";

export const size = { width: 32, height: 32 };
export const contentType = "image/png";

export default function Icon() {
  return new ImageResponse(
    <div
      style={{
        width: "100%",
        height: "100%",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        // rgb(63, 105, 211) — matches --color-primary in libs/web-ui-token/src/beaver-nest.css
        // (Satori/ImageResponse can't read CSS custom properties, so the resolved value is hardcoded)
        background: "#3f69d3",
        color: "#ffffff",
        fontSize: 20,
        fontWeight: 800,
        borderRadius: 6,
      }}
    >
      B
    </div>,
    { ...size },
  );
}
