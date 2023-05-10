import { deserialize, Schema } from "borsh";
import { Connection, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import {
  getHashedNameSync,
  getNameAccountKeySync,
} from "@bonfida/spl-name-service";

export const CATEGORY_TLD = new PublicKey(
  "7Eg3kuaLtyGaKBBHiQm4YGGTmyrADrkWiMPzUuUdLua9"
);

export enum Tag {
  Uninitialized,
  CategoryMetadata,
  CategoryMetadataClosed,
  CategoryMember,
  CategoryMemberClosed,
}

export class CategoryMember {
  tag: Tag;
  name: string;
  domainKey: PublicKey;

  static schema: Schema = new Map([
    [
      CategoryMember,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["domainKey", [32]],
          ["name", "string"],
        ],
      },
    ],
  ]);

  constructor(obj: { tag: number; name: string; domainKey: Uint8Array }) {
    this.tag = obj.tag as Tag;
    this.name = obj.name;
    this.domainKey = new PublicKey(obj.domainKey);
  }

  static deserialize(data: Buffer): CategoryMember {
    return deserialize(this.schema, CategoryMember, data);
  }

  static async retrieve(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);
    if (!accountInfo || !accountInfo.data) {
      throw new Error("State account not found");
    }
    return this.deserialize(accountInfo.data);
  }
  static findKey(domainName: string, category: PublicKey) {
    const hashed = getHashedNameSync(domainName);
    const key = getNameAccountKeySync(hashed, undefined, category);
    return key;
  }
}

export class CategoryMetadata {
  tag: Tag;
  nbRegisteredDomain: number;
  name: string;

  static schema: Schema = new Map([
    [
      CategoryMember,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["nbRegisteredDomain", "u32"],
          ["name", "string"],
        ],
      },
    ],
  ]);

  constructor(obj: { tag: number; nbRegisteredDomain: number; name: string }) {
    this.tag = obj.tag as Tag;
    this.nbRegisteredDomain = obj.nbRegisteredDomain;
    this.name = obj.name;
  }

  static deserialize(data: Buffer): CategoryMember {
    return deserialize(this.schema, CategoryMember, data);
  }

  static async retrieve(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);
    if (!accountInfo || !accountInfo.data) {
      throw new Error("State account not found");
    }
    return this.deserialize(accountInfo.data);
  }
  static findKey(categoryName: string) {
    const hashed = getHashedNameSync(categoryName);
    const key = getNameAccountKeySync(hashed, undefined, CATEGORY_TLD);
    return key;
  }
}
