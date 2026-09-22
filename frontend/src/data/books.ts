// src/data/books.ts

export interface Author {
  name: string;
}

export interface Category {
  name: string;
}

export interface ApiPagination {
  total_items: number;
  offset: number;
  limit: number;
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
  pagination: ApiPagination;
  livres: ApiBook[];
}

export interface Comment {
  id: string;
  author: string;
  date: string;
  content: string;
}

export interface UserReading {
  id: number;
  user_id: number;
  book_id: string;
  note: number | null;
  read_at: string;
}

export interface ApiComment {
  id: number;
  content: string;
  user_id: number;
  book_id: string;
  parent_id?: number | null;
  created_at: string;
}

export interface ApiCommentWithReplies {
  id: number;
  content: string;
  user_id: number;
  book_id: string;
  created_at: string;
  reponses: ApiComment[];
}