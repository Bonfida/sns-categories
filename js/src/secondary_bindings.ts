import { Connection, MemcmpFilter, PublicKey } from "@solana/web3.js";
import { CATEGORY_TLD, Tag } from "./state";
import { NAME_PROGRAM_ID, NameRegistryState } from "@bonfida/spl-name-service";

export const getAllCategories = async (connection: Connection) => {
  const filters: MemcmpFilter[] = [
    {
      memcmp: {
        offset: 0,
        bytes: CATEGORY_TLD.toBase58(),
      },
    },
    {
      memcmp: {
        offset: NameRegistryState.HEADER_LEN,
        bytes: ((Tag.CategoryMetadata as number) + 1).toString(),
      },
    },
  ];

  const result = await connection.getProgramAccounts(NAME_PROGRAM_ID, {
    filters,
  });

  return result;
};

export const getAllMembers = async (
  connection: Connection,
  category: PublicKey
) => {
  const filters: MemcmpFilter[] = [
    {
      memcmp: {
        offset: 0,
        bytes: category.toBase58(),
      },
    },
    {
      memcmp: {
        offset: NameRegistryState.HEADER_LEN,
        bytes: ((Tag.CategoryMember as number) + 1).toString(),
      },
    },
  ];

  const result = await connection.getProgramAccounts(NAME_PROGRAM_ID, {
    filters,
  });

  return result;
};
