import { Connection, MemcmpFilter, PublicKey } from "@solana/web3.js";
import { CATEGORY_TLD, CategoryMember, CategoryMetadata, Tag } from "./state";
import { NAME_PROGRAM_ID, NameRegistryState } from "@bonfida/spl-name-service";
import { SNS_CATEGORIES_ID } from "./bindings";

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
        bytes: (Tag.CategoryMetadata + 1).toString(),
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

export const verifyDomain = async (
  connection: Connection,
  domain: string,
  category: string
) => {
  const categoryKey = CategoryMetadata.findKey(category);
  const memberKey = CategoryMember.findKey(domain, categoryKey);
  const info = await connection.getAccountInfo(memberKey);
  return !!info?.data;
};

export const verifyDomainFromMemberKey = async (
  connection: Connection,
  member: PublicKey,
  category: string
) => {
  const { registry } = await NameRegistryState.retrieve(connection, member);
  const categoryKey = CategoryMetadata.findKey(category);
  return registry.parentName.equals(categoryKey);
};

export const verifyDomainFromDomainKey = async (
  connection: Connection,
  domain: PublicKey,
  category: string
) => {
  const categoryKey = await CategoryMetadata.findKey(category);
  const filters: MemcmpFilter[] = [
    {
      memcmp: {
        offset: 0,
        bytes: categoryKey.toBase58(),
      },
    },
    {
      memcmp: {
        offset: NameRegistryState.HEADER_LEN + 1,
        bytes: domain.toBase58(),
      },
    },
    {
      memcmp: {
        offset: NameRegistryState.HEADER_LEN,
        bytes: (Tag.CategoryMember + 1).toString(),
      },
    },
  ];
  const res = await connection.getProgramAccounts(SNS_CATEGORIES_ID, {
    filters,
  });
  return res.length === 1;
};
