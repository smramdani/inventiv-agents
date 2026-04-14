-- Allow login to resolve a user by email without prior org context (RLS-safe).
-- Registration sets org context in-app; this function is DEFINER-only for the lookup row.

CREATE OR REPLACE FUNCTION public.lookup_user_for_login(p_email TEXT)
RETURNS TABLE(user_id UUID, organization_id UUID, role_name TEXT)
LANGUAGE sql
SECURITY DEFINER
SET search_path = public
AS $$
  SELECT u.id, u.organization_id, u.role::text
  FROM users u
  WHERE u.email = p_email
  LIMIT 1;
$$;

REVOKE ALL ON FUNCTION public.lookup_user_for_login(TEXT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.lookup_user_for_login(TEXT) TO inventiv_app;
