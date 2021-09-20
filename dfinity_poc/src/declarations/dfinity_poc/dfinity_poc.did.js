export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_public_key' : IDL.Func([], [IDL.Text], []),
    'store_message' : IDL.Func([IDL.Text, IDL.Text, IDL.Text], [IDL.Text], []),
    'store_pub_key' : IDL.Func([IDL.Text, IDL.Text, IDL.Text], [IDL.Text], []),
    'update_key' : IDL.Func([IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
