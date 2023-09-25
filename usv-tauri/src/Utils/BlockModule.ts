export interface Block {
    zone_holder: String;
    actions: Action[];
}

export interface Action {
    taunter: String;
    word: String;
}

export class BlockModule {
    blocks: Block[];
    current_block!: Block;
    index = 0;

    constructor(blocks: Block[]) {
        this.blocks = blocks;
        this.current_block = blocks[this.index]
    }

    Next() {     
        if (this.index == this.blocks.length) {
            this.index = 0
        } else {
            this.index++
        }
        this.current_block = this.blocks[this.index]
    }
}