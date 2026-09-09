// src/pages/logout.ts
import type { APIRoute } from 'astro';

export const prerender = false;

export const POST: APIRoute = async ({ cookies, redirect }) => {
  cookies.delete('auth_token', { path: '/' });

  return redirect('/login');
};