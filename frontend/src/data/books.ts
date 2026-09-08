// src/data/books.ts

export interface Comment {
  id: string;
  author: string;
  date: string;
  content: string;
}

export interface Book {
  id: string;
  title: string;
  author: string;
  year: number;
  rating: number; // 1 to 5
  isStaffPick: boolean;
  coverImage: string;
  review: string;
  highlights: string[];
}

export const booksList: Book[] = [
  {
    id: "dune",
    title: "Dune",
    author: "Frank Herbert",
    year: 1965,
    rating: 5,
    isStaffPick: true,
    coverImage: "https://images.unsplash.com/photo-1544716278-ca5e3f4abd8c?w=600&auto=format&fit=crop&q=80",
    review: "Un monument de la science-fiction. L'écologie rude d'Arrakis, le mysticisme et les querelles dynastiques créent une immersion totale.",
    highlights: ["Écologie planétaire & vers géants", "Intrigues politiques impériales", "Mysticisme du Bene Gesserit"]
  },
  {
    id: "1984",
    title: "1984",
    author: "George Orwell",
    year: 1949,
    rating: 4,
    isStaffPick: true,
    coverImage: "https://images.unsplash.com/photo-1512820790803-83ca734da794?w=600&auto=format&fit=crop&q=80",
    review: "Une dystopie glaçante et indémodable. La manipulation de la vérité et la surveillance permanente résonnent avec une acuité terrifiante.",
    highlights: ["La novlangue et le contrôle mental", "Atmosphère étouffante", "Le Ministère de la Vérité"]
  },
  {
    id: "foundation",
    title: "Fondation",
    author: "Isaac Asimov",
    year: 1951,
    rating: 5,
    isStaffPick: false,
    coverImage: "https://images.unsplash.com/photo-1532012164546-f432f2e3777f?w=600&auto=format&fit=crop&q=80",
    review: "Une saga galactique intellectuelle fondée sur la psychohistoire. Prédire la chute de civilisations sans verser une seule goutte de sang inutile.",
    highlights: ["Chute d'un empire millénaire", "Le sanctuaire de Terminus", "La force de l'intelligence"]
  },
  {
    id: "the-lord-of-the-rings",
    title: "Le Seigneur des Anneaux",
    author: "J.R.R. Tolkien",
    year: 1954,
    rating: 5,
    isStaffPick: true,
    coverImage: "https://images.unsplash.com/photo-1463320726281-696a485928c7?w=600&auto=format&fit=crop&q=80",
    review: "Le sommet de la fantasy universelle. Chaque recoin de la Terre du Milieu semble avoir existé avant même que l'auteur ne l'écrive.",
    highlights: ["Worldbuilding inégalé", "Légendaire elfique et nain", "Quête fraternelle inoubliable"]
  }
];

export const commentsByBookId: Record<string, Comment[]> = {
  "dune": [
    {
      id: "c1",
      author: "Paul",
      date: "10/02/2026",
      content: "L'épice doit couler ! Un univers d'une richesse incomparable."
    },
    {
      id: "c2",
      author: "Dame Jessica",
      date: "14/03/2026",
      content: "La dimension philosophique et politique transcende la simple SF."
    }
  ],
  "1984": [
    {
      id: "c3",
      author: "Winston Smith",
      date: "04/04/2026",
      content: "Une lecture indispensable mais terrifiante de lucidité."
    }
  ],
  "foundation": [
    {
      id: "c4",
      author: "Hari Seldon",
      date: "20/01/2026",
      content: "Le concept de psychohistoire reste encore aujourd'hui fascinant."
    },
    {
      id: "c5",
      author: "Salvor Hardin",
      date: "15/02/2026",
      content: "La violence est le dernier refuge de l'incompétence. Culte absolu."
    }
  ],
  "the-lord-of-the-rings": [
    {
      id: "c6",
      author: "Frodon",
      date: "01/01/2026",
      content: "Une aventure monumentale. On ne ressort pas indemne de ce périple."
    }
  ]
};