// src/data/books.ts

export interface Author {
  name: string;
}

export interface Category {
  name: string;
}

export interface ApiBook {
  id: string;
  title: string;
  authors: Author[];
  description: string | null;
  image_url: string | null;
  page_count: number;
  categories: Category[];
  average_rating: number;
  ratings_count: number;
  maturity_rating: string;
  published_date: string;
}

export interface ApiResponse {
  pagination: {
    total_items: number;
    offset: number;
    limit: number;
  };
  livres: ApiBook[];
}

export interface Comment {
  id: string;
  author: string;
  date: string;
  content: string;
}

export const mockApiResponse: ApiResponse = {
  pagination: {
    total_items: 300,
    offset: 10,
    limit: 10
  },
  livres: [
    {
      id: "Vcj7467VJ0cC",
      title: "Catalogue of the Printed Books in the Library of the Faculty of Advocates",
      authors: [{ name: "Advocates' Library" }],
      description: null,
      image_url: "http://books.google.com/books/content?id=Vcj7467VJ0cC&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api",
      page_count: 276,
      categories: [{ name: "Academic libraries" }],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1867"
    },
    {
      id: "oCPmtwEACAAJ",
      title: "Le Livre de la Ferme et des Maisons de Campagne. Tome 1",
      authors: [{ name: "Sans Auteur" }],
      description: "Le livre de la famille: les personnes et les choses, savoir-vivre et savoir-faire, morale, education, economie domestique, hygiene, soins aux enfants, etc.",
      image_url: "http://books.google.com/books/content?id=oCPmtwEACAAJ&printsec=frontcover&img=1&zoom=1&source=gbs_api",
      page_count: 1048,
      categories: [],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "2018-02-28"
    },
    {
      id: "noJMAQAAMAAJ",
      title: "Catalogue",
      authors: [{ name: "Sotheby, Wilkinson & Hodge" }],
      description: null,
      image_url: "http://books.google.com/books/content?id=noJMAQAAMAAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api",
      page_count: 260,
      categories: [{ name: "Art" }],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1887"
    },
    {
      id: "Zs8mAQAAIAAJ",
      title: "Industrie Du Livre Au Canada, Avec Où Trouver Qui",
      authors: [],
      description: null,
      image_url: "http://books.google.com/books/content?id=Zs8mAQAAIAAJ&printsec=frontcover&img=1&zoom=1&source=gbs_api",
      page_count: 436,
      categories: [{ name: "Book industries and trade" }],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1993"
    },
    {
      id: "n2VWAAAAYAAJ",
      title: "A General Catalogue of Books Offered to the Public at the Affixed Prices by Bernard Quaritch ...",
      authors: [],
      description: null,
      image_url: "http://books.google.com/books/content?id=n2VWAAAAYAAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api",
      page_count: 594,
      categories: [{ name: "Booksellers' catalogs" }],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1883"
    },
    {
      id: "Nd__0AEACAAJ",
      title: "Le grand livre jeu de Noël",
      authors: [],
      description: null,
      image_url: null,
      page_count: 0,
      categories: [],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "2024-11-08"
    },
    {
      id: "IA9FAQAAMAAJ",
      title: "The Book of Ser Marco Polo, the Venetian",
      authors: [{ name: "Marco Polo" }],
      description: null,
      image_url: "http://books.google.com/books/content?id=IA9FAQAAMAAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api",
      page_count: 612,
      categories: [{ name: "Asia" }],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1871"
    },
    {
      id: "-8wzAAAAIAAJ",
      title: "A Treatise on the Effect of the Contract of Sale on the Legal Rights of Property and Possession in Goods, Wares, and Merchandise",
      authors: [{ name: "Colin Blackburn Baron Blackburn" }],
      description: null,
      image_url: "http://books.google.com/books/content?id=-8wzAAAAIAAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api",
      page_count: 476,
      categories: [{ name: "Sales" }],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1896"
    },
    {
      id: "tokfzgEACAAJ",
      title: "Mon Premier Bébé Livre D'or",
      authors: [{ name: "Livre d'or Premier bébé Publishing" }],
      description: "Le premier livre d'or pour la naissance du premier bébé, 50 pages à remplir par les invités pour donner des conseils au couple et souhaiter le meilleur au nouveau né.",
      image_url: null,
      page_count: 50,
      categories: [],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "2020-08-14"
    },
    {
      id: "Eh7MeM15SfEC",
      title: "Mathesis",
      authors: [],
      description: null,
      image_url: "http://books.google.com/books/content?id=Eh7MeM15SfEC&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api",
      page_count: 356,
      categories: [],
      average_rating: 0,
      ratings_count: 0,
      maturity_rating: "NOT_MATURE",
      published_date: "1891"
    }
  ]
};

export const commentsByBookId: Record<string, Comment[]> = {
  // Catalogue of the Printed Books...
  "Vcj7467VJ0cC": [
    {
      id: "c1",
      author: "Paul",
      date: "10/02/2026",
      content: "Un catalogue d'archives fascinant pour quiconque s'intéresse à l'histoire des bibliothèques juridiques écossaises."
    },
    {
      id: "c2",
      author: "Dame Jessica",
      date: "14/03/2026",
      content: "Une mine d'or bibliographique, particulièrement rare et détaillée."
    }
  ],
  // Le Livre de la Ferme...
  "oCPmtwEACAAJ": [
    {
      id: "c3",
      author: "Winston Smith",
      date: "04/04/2026",
      content: "Un témoignage passionnant du quotidien rural et des savoirs pratiques de la fin du XIXe siècle."
    }
  ],
  // The Book of Ser Marco Polo...
  "IA9FAQAAMAAJ": [
    {
      id: "c4",
      author: "Hari Seldon",
      date: "20/01/2026",
      content: "Les récits de voyages marchands restent le socle des chroniques d'exploration médiévale."
    },
    {
      id: "c5",
      author: "Salvor Hardin",
      date: "15/02/2026",
      content: "Une édition historique magistrale des périples de Marco Polo."
    }
  ],
  // Mon Premier Bébé Livre D'or
  "tokfzgEACAAJ": [
    {
      id: "c6",
      author: "Frodon",
      date: "01/01/2026",
      content: "Un carnet de souvenirs doux et soigné pour immortaliser de précieux moments."
    }
  ]
};

export const booksList = mockApiResponse.livres;