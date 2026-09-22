// src/pages/api/comments.ts
import type { APIRoute } from 'astro';
import { API_URL } from 'astro:env/server';

export const prerender = false;

export const POST: APIRoute = async ({ request, cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    return new Response(
      JSON.stringify({ message: 'Vous devez être connecté pour publier un commentaire.' }),
      { status: 401, headers: { 'Content-Type': 'application/json' } }
    );
  }

  try {
    const body = await request.json();
    const { book_id, content, parent_id } = body;

    if (!content || !content.trim()) {
      return new Response(
        JSON.stringify({ message: 'Le contenu du commentaire ne peut pas être vide.' }),
        { status: 400, headers: { 'Content-Type': 'application/json' } }
      );
    }

    if (!book_id) {
      return new Response(
        JSON.stringify({ message: 'L’identifiant du livre est requis.' }),
        { status: 400, headers: { 'Content-Type': 'application/json' } }
      );
    }

    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const backendRes = await fetch(`${baseUrl}/comments`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: JSON.stringify({
        book_id,
        content: content.trim(),
        parent_id: parent_id ? Number(parent_id) : null,
      }),
    });

    const data = await backendRes.json().catch(() => ({}));

    return new Response(JSON.stringify(data), {
      status: backendRes.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (err: any) {
    return new Response(
      JSON.stringify({ message: err.message || 'Erreur serveur lors de la publication du commentaire.' }),
      { status: 500, headers: { 'Content-Type': 'application/json' } }
    );
  }
};

