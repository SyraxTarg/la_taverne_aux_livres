// src/pages/api/readings.ts
import type { APIRoute } from 'astro';
import { API_URL } from 'astro:env/server';

export const prerender = false;

export const PUT: APIRoute = async ({ request, cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    console.warn('[API /api/readings] Aucun auth_token trouvé dans les cookies');
    return new Response(
      JSON.stringify({ erreur: 'Vous devez être connecté pour noter un livre.' }),
      {
        status: 401,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }

  try {
    const body = await request.json();
    const { book_id, note } = body;

    let finalNote: number | null = null;
    if (note !== null && note !== undefined && note !== '') {
      const noteInt = typeof note === 'number' ? Math.round(note) : parseInt(String(note), 10);
      if (!isNaN(noteInt)) {
        finalNote = noteInt;
      }
    }

    const payload = {
      book_id: String(book_id),
      note: finalNote,
    };

    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const endpoint = `${baseUrl}/readings`;

    console.log(`[API /api/readings] Relais PUT vers ${endpoint} avec body:`, JSON.stringify(payload));

    const response = await fetch(endpoint, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: JSON.stringify(payload),
    });

    console.log(`[API /api/readings] Réponse du backend statut : ${response.status}`);

    let data: any;
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
      data = await response.json();
    } else {
      const text = await response.text();
      data = { erreur: text || `Erreur serveur (${response.status})` };
    }

    return new Response(JSON.stringify(data), {
      status: response.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (error: any) {
    console.error('[API /api/readings] Erreur lors du relais :', error);
    return new Response(
      JSON.stringify({ erreur: error.message || 'Impossible de contacter le serveur backend.' }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }
};

export const DELETE: APIRoute = async ({ request, cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    console.warn('[API DELETE /api/readings] Aucun auth_token trouvé dans les cookies');
    return new Response(
      JSON.stringify({ erreur: 'Vous devez être connecté pour supprimer une note.' }),
      {
        status: 401,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }

  try {
    let book_id: string | null = null;
    const url = new URL(request.url);
    book_id = url.searchParams.get('book_id');

    if (!book_id) {
      try {
        const body = await request.json();
        book_id = body.book_id;
      } catch {}
    }

    if (!book_id) {
      return new Response(
        JSON.stringify({ erreur: 'Identifiant du livre manquant.' }),
        {
          status: 400,
          headers: { 'Content-Type': 'application/json' },
        }
      );
    }

    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const endpoint = `${baseUrl}/readings/${encodeURIComponent(book_id)}`;

    console.log(`[API DELETE /api/readings] Relais DELETE vers ${endpoint}`);

    const response = await fetch(endpoint, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    console.log(`[API DELETE /api/readings] Réponse du backend statut : ${response.status}`);

    let data: any;
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
      data = await response.json();
    } else {
      const text = await response.text();
      data = { message: text || `Statut ${response.status}` };
    }

    return new Response(JSON.stringify(data), {
      status: response.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (error: any) {
    console.error('[API DELETE /api/readings] Erreur lors du relais :', error);
    return new Response(
      JSON.stringify({ erreur: error.message || 'Impossible de contacter le serveur backend.' }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }
};
