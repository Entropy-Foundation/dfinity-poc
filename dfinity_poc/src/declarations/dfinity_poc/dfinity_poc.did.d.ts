import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'get_public_key' : () => Promise<string>,
  'store_message' : (arg_0: string, arg_1: string, arg_2: string) => Promise<
      string
    >,
  'store_pub_key' : (arg_0: string, arg_1: string, arg_2: string) => Promise<
      string
    >,
  'update_key' : (arg_0: string) => Promise<undefined>,
}
