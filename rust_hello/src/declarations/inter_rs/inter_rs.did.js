export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'store_message' : IDL.Func([IDL.Text, IDL.Text, IDL.Text], [IDL.Text], []),
    'update_key' : IDL.Func([IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
