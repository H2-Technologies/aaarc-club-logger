let supabase = window.supabase;

async function initSupabaseClient() {
  Client = await supabase.createClient(
    "https://bogabjuetzwjxopxtgul.supabase.co",
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImJvZ2FianVldHp3anhvcHh0Z3VsIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDM0NDIwMTMsImV4cCI6MjAxOTAxODAxM30.8kWWroSpdePixUAM8_gtSvYhxmSnry9_cl_npkSmI1w"
  );
  return Client;
}

async function signUp(email, password, callsign) {
  let supabase = await initSupabaseClient();
  let { data, error } = await supabase.auth.signUp({
    email: email,
    password: password,
    options: {
      data: {
        callsign: callsign,
      },
    },
  });
  return { data, error };
}

async function signIn(email, password) {
  let supabase = await initSupabaseClient();
  let { data, error } = await supabase.auth.signIn({
    email: email,
    password: password,
  });
  return { data, error };
}

async function signOut() {
  let supabase = await initSupabaseClient();
  let { error } = await supabase.auth.signOut();
  return { error };
}

async function getUser() {
  let supabase = await initSupabaseClient();
  let user = await supabase.auth.user();
  return user;
}

async function getCallsign() {
  let supabase = await initSupabaseClient();
  let user = await supabase.auth.user();
  return user.user_metadata.callsign;
}

async function getEvents() {
	let supabase = await initSupabaseClient();
	let user = await supabase.auth.user();
	return user.user_metadata.events;
}