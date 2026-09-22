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

    const noteInt = typeof note === 'number' ? Math.round(note) : parseInt(String(note), 10);
    const payload = {
      book_id: String(book_id),
      note: isNaN(noteInt) ? null : noteInt,
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
