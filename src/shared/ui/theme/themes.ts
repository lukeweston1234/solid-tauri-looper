export interface ThemeItem {
  name: string;
  theme: Theme;
}

export interface Theme {
  primaryColor: string;
  backgroundColor: string;
  leftLight: string;
  rightLight: string;
  timeMarker: string;
}

export const themes: ThemeItem[] = [
  {
    name: "Kombu",
    theme: {
      primaryColor: "#D3FECC",
      backgroundColor: "#000000",
      leftLight: "rgba(0,0,0,0)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(67, 122, 116, 0.21) 0%, rgba(33, 90, 102, 0.09) 100%)",
      timeMarker: "#8FFA6A",
    },
  },
  {
    name: "Dust",
    theme: {
      primaryColor: "#FAD16A",
      backgroundColor: "#000000",
      leftLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(247, 225, 151, 0.133) 0%, rgba(136, 95, 65, 0) 100%)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(220, 144, 101, 0.21) 0%, rgba(134, 66, 45, 0.09) 100%)",
      timeMarker: "#FAD16A",
    },
  },
  {
    name: "Crush",
    theme: {
      primaryColor: "#FFC0CB",
      backgroundColor: "#3D1F2D",
      leftLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(255, 192, 203, 0.2) 0%, rgba(61, 31, 45, 0.1) 100%)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(255, 192, 203, 0.3) 0%, rgba(61, 31, 45, 0.1) 100%)",
      timeMarker: "#FFC0CB",
    },
  },
  {
    name: "Arasaka",
    theme: {
      primaryColor: "#FA6A6A",
      backgroundColor: "#000000",
      leftLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(237, 186, 170, 0.133) 0%, rgba(255, 92, 70, 0) 100%)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(152, 123, 166, 0.21) 0%, rgba(59, 33, 102, 0.09) 100%)",
      timeMarker: "#FA6A6A",
    },
  },
  {
    name: "Ember",
    theme: {
      primaryColor: "#FF4500",
      backgroundColor: "#1C0F0A",
      leftLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(255, 69, 0, 0.2) 0%, rgba(28, 15, 10, 0.1) 100%)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(204, 55, 0, 0.3) 0%, rgba(28, 15, 10, 0.1) 100%)",
      timeMarker: "#FF4500",
    },
  },
  {
    name: "Phantom",
    theme: {
      primaryColor: "#6D6875",
      backgroundColor: "#1B1B1F",
      leftLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(109, 104, 117, 0.2) 0%, rgba(27, 27, 31, 0.1) 100%)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(88, 85, 96, 0.3) 0%, rgba(27, 27, 31, 0.1) 100%)",
      timeMarker: "#6D6875",
    },
  },
  {
    name: "Cinder",
    theme: {
      primaryColor: "#FF9F1C",
      backgroundColor: "#2D2D2D",
      leftLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(255, 159, 28, 0.2) 0%, rgba(45, 45, 45, 0.1) 100%)",
      rightLight:
        "radial-gradient(50% 50% at 50% 50%, rgba(204, 127, 22, 0.3) 0%, rgba(45, 45, 45, 0.1) 100%)",
      timeMarker: "#FF9F1C",
    },
  },
];
